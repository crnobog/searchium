import * as vscode from 'vscode';
import { DocumentRegistrationService } from './documentRegistrationService';
import { getLogger } from './logger';
import { SearchResultsProvider, SearchManager } from './search';
import { ControlsProvider } from './controlsProvider';
import { IndexState } from './indexState';
import { DetailsPanelProvider } from './detailsPanel';
import { SearchHistory } from './history';
import { startServer } from './index/indexServerProcess';
import { startLegacyServer } from './index/legacy/legacyServerProcess';
import { IpcChannel } from 'ipcChannel';
import * as ipcEvents from 'ipcEvents';
import { FileSearchManager } from 'fileSearch';

async function drainIndexProgress(
    progress: vscode.Progress<{ increment: number, message: string }>,
    event: ipcEvents.ProgressReportEvent,
    iterator: AsyncIterator<ipcEvents.ProgressReportEvent>
): Promise<void> {
    const total = event.total;
    let completed = event.completed;
    if (completed === total) { return; }
    getLogger().logInformation`Starting index progress ${completed} ... ${total}`;
    progress.report({ increment: completed / total * 100, message: event.displayText });
    for (; ;) {
        const i = await iterator.next();
        if (i.done) { return; }
        const e = i.value;
        if (e.total !== total) {
            getLogger().logInformation`Indexing aborted ${e.total} not equal ${total}`;
            return;
        }
        getLogger().logInformation`Continuing index progress ${completed} ... ${total}`;
        progress.report({ increment: (e.completed - completed) / e.total * 100, message: e.displayText });
        completed = e.completed;
        if (e.completed === e.total) {
            getLogger().logInformation`Indexing complete ${e.completed}`;
            return;
        }
    }
}

class IndexProgressReporter implements vscode.Disposable {
    abort = false;
    constructor(private channel: IpcChannel) {
        this.startListening();
    }

    public dispose(): void {
        this.abort = true;
    }

    private getIndexProgress(): AsyncIterable<ipcEvents.ProgressReportEvent> {
        const channel = this.channel;
        return async function* () {
            let events: ipcEvents.ProgressReportEvent[] = [];
            let resolve: () => void;
            let promise: Promise<void> = new Promise<void>(r => resolve = r);

            channel.on('event', (e: ipcEvents.TypedEvent) => {
                if (e.eventType === 'progressReport') {
                    events.push(e);
                    resolve();
                    promise = new Promise<void>(r => resolve = r);
                }
            });
            for (; ;) {
                await promise;
                yield* events;
                events = [];
            }
        }();
    }

    private async startListening(): Promise<void> {
        const iterable = this.getIndexProgress();
        const iterator = iterable[Symbol.asyncIterator]();
        getLogger().logInformation`IndexProgressReporter.startListening`;
        for (; ;) {
            const i = await iterator.next();
            if (i.done || this.abort) { break; }
            const e: ipcEvents.ProgressReportEvent = i.value;
            const options: vscode.ProgressOptions = {
                location: vscode.ProgressLocation.Notification,
                cancellable: false,
                title: "Searchium Indexing"
            };
            try {
                getLogger().logInformation`Starting progress for ${e.total}`;
                await vscode.window.withProgress(options, (progress, _token): Promise<void> => {
                    return drainIndexProgress(progress, e, iterator);
                });
                getLogger().logInformation`Progress complete`;
            } catch (error) {
                getLogger().logInformation`Progress aborted`;
                continue;
            }
        }
    }
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    try {
        getLogger().logInformation`Initializing searchium`;
        const config = vscode.workspace.getConfiguration("searchium");
        const useLegacyServer = config.get<boolean>("useLegacyServer", true);
        let searchManager, searchResultsProvider, controlsProvider, detailsPanelProvider;

        const history = new SearchHistory(context);
        if (useLegacyServer) {
            const [proxy, channel] = await startLegacyServer(context);
            context.subscriptions.push(proxy);

            channel.on('response', (r) => {
                switch (r.responseType) {
                    default:
                        getLogger().logDebug`response: ${r}`;
                        break;
                    case 'searchCode':
                        break;
                }
            });
            channel.on('event', (e) => getLogger().logDebug`event: ${e}`);

            const indexState = new IndexState(channel);

            searchResultsProvider = new SearchResultsProvider(channel);
            context.subscriptions.push(vscode.window.registerTreeDataProvider("searchium-results", searchResultsProvider));
            const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
                { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
            searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, channel, history);

            controlsProvider = new ControlsProvider(context, context.extensionUri, history, indexState);

            context.subscriptions.push(new IndexProgressReporter(channel));
            context.subscriptions.push(new DocumentRegistrationService(context, channel, undefined));

            // const fileSearchManager = new FileSearchManager(channel);
            detailsPanelProvider = new DetailsPanelProvider(context, channel);

            context.subscriptions.push(
                vscode.commands.registerCommand("searchium.query", searchManager.onQuery, searchManager),
                vscode.commands.registerCommand('searchium.nextResult', searchManager.navigateToNextResult, searchManager),
                vscode.commands.registerCommand('searchium.previousResult', searchManager.navigateToPreviousResult, searchManager),

                // Not working very well with chromium server
                // vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager),

                // vscode.commands.registerCommand("searchium.openDetails", detailsPanelProvider.openDetails, detailsPanelProvider),

                vscode.commands.registerCommand("searchium.toggleCaseSensitivity", controlsProvider.onToggleCaseSensitivity, controlsProvider),
                vscode.commands.registerCommand("searchium.toggleWholeWord", controlsProvider.onToggleWholeWord, controlsProvider),
                vscode.commands.registerCommand("searchium.toggleRegex", controlsProvider.onToggleRegex, controlsProvider),
                vscode.commands.registerCommand("searchium.previousQuery", controlsProvider.onPreviousQuery, controlsProvider),
                vscode.commands.registerCommand("searchium.nextQuery", controlsProvider.onNextQuery, controlsProvider),
            );
        }
        else {
            // TODO: Progress bar/status bar update for this? 
            const [process, client] = await startServer(context);
            const fileSearchManager = new FileSearchManager(client);
            searchResultsProvider = new SearchResultsProvider(client);
            const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
                { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
            searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, client, history);
            const indexState = new IndexState(client);
            controlsProvider = new ControlsProvider(context, context.extensionUri, history, indexState);
            detailsPanelProvider = new DetailsPanelProvider(context, client);
            context.subscriptions.push(
                process,
                new DocumentRegistrationService(context, undefined, client),
                fileSearchManager,
                vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager)
            );

            const toMbString = (value: bigint): string =>
                `${(Number(value / 1024n) / 1024.0).toFixed(2)} MB`;
            setInterval(async () => {
                const info = await client.getProcessInfo();
                getLogger().logInformation`Index server physical memory: ${toMbString(info.physicalMemory)} Virtual memory: ${toMbString(info.virtualMemory)}`;
            }, 10 * 1000);
        }
        context.subscriptions.push(
            vscode.window.registerWebviewViewProvider("searchium-controls", controlsProvider),

            vscode.commands.registerCommand("searchium.query", searchManager.onQuery, searchManager),
            vscode.commands.registerCommand('searchium.nextResult', searchManager.navigateToNextResult, searchManager),
            vscode.commands.registerCommand('searchium.previousResult', searchManager.navigateToPreviousResult, searchManager),

            vscode.commands.registerCommand("searchium.openDetails", detailsPanelProvider.openDetails, detailsPanelProvider),

            // todo: rename commands 
            vscode.commands.registerCommand("searchium.focusSearch", controlsProvider.onJumpToSearchInput, controlsProvider),
            vscode.commands.registerCommand("searchium.newSearch", controlsProvider.onNewSearch, controlsProvider),
            vscode.commands.registerCommand("searchium.clearHistory", controlsProvider.onClearHistory, controlsProvider),
            vscode.commands.registerTextEditorCommand("searchium.searchCurrentToken", controlsProvider.onSearchCurrentToken, controlsProvider),

            // vscode.commands.registerCommand("searchium.toggleCaseSensitivity", controlsProvider.onToggleCaseSensitivity, controlsProvider),
            // vscode.commands.registerCommand("searchium.toggleWholeWord", controlsProvider.onToggleWholeWord, controlsProvider),
            // vscode.commands.registerCommand("searchium.toggleRegex", controlsProvider.onToggleRegex, controlsProvider),
            // vscode.commands.registerCommand("searchium.previousQuery", controlsProvider.onPreviousQuery, controlsProvider),
            // vscode.commands.registerCommand("searchium.nextQuery", controlsProvider.onNextQuery, controlsProvider),
        );
    } catch (err) {
        getLogger().logError`Unexpected error initializing extension: ${err}`;
    }
}

export function deactivate(): void {
    getLogger().logInformation`Deactivating searchium extension`;
}
