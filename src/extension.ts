import * as vscode from 'vscode';
import { DocumentRegistrationService } from './documentRegistrationService';
import * as ipcEvents from './ipcEvents';
import { IpcChannel } from './ipcChannel';
import { getLogger } from './logger';
import { SearchResultsProvider, SearchManager } from './search';
import { ControlsProvider } from './controlsProvider';
import { IndexState } from './indexState';
import { DetailsPanelProvider } from './detailsPanel';
import { SearchHistory } from './history';
import { startServer } from './index/indexServerProcess';
import { startLegacyServer } from './index/legacy/legacyServerProcess';

class IndexProgressInstance {
    total: number;
    completed: number;
    listener?: (e: ipcEvents.TypedEvent) => void;

    constructor(event: ipcEvents.ProgressReportEvent, private channel: IpcChannel) {
        this.total = event.total;
        this.completed = event.completed;
    }

    public async run(_event: ipcEvents.ProgressReportEvent): Promise<void> {
        const options: vscode.ProgressOptions = {
            location: vscode.ProgressLocation.Notification,
            cancellable: false,
            title: "Searchium Indexing"
        };
        await vscode.window.withProgress(options, this.progress.bind(this));
    }

    private async progress(progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken): Promise<void> {
        progress.report({ increment: this.completed / this.total * 100, message: "" });
        if (this.completed === this.total) {
            return Promise.resolve();
        }
        return new Promise<void>((resolve, reject) => {
            const listener = (e: ipcEvents.TypedEvent): void => {
                if (e.eventType !== 'progressReport') { return; }

                // If total changed, start a new progress window
                if (e.total !== this.total) {
                    this.channel.off('event', listener);
                    reject();
                    return;
                }

                progress.report({ increment: (this.completed - e.completed) / e.total * 100, message: e.displayText });
                this.completed = e.completed;
                if (e.completed === e.total) {
                    resolve();
                    this.channel.off('event', listener);
                }
            };
            this.channel.on('event', listener);
        });
    }
}

class IndexProgressReporter implements vscode.Disposable {
    listener: (event: ipcEvents.TypedEvent) => void;
    constructor(private channel: IpcChannel) {
        this.listener = this.onIpcEvent.bind(this);
        this.startListening();
    }

    public dispose(): void {
        this.stopListening();
    }

    private onIpcEvent(event: ipcEvents.TypedEvent): void {
        if (event.eventType === 'progressReport') {
            this.startProgress(event);
        }
    }

    private startListening(): void {
        getLogger().logDebug`IndexProgressReporter: start listening`;
        this.channel.on('event', this.listener);
    }
    private stopListening(): void {
        getLogger().logDebug`IndexProgressReporter: stop listening`;
        this.channel.off('event', this.listener);
    }

    private async startProgress(event: ipcEvents.ProgressReportEvent): Promise<void> {
        getLogger().logDebug`IndexProgressReporter: starting progress report total ${event.total}`;
        const reporter = new IndexProgressInstance(event, this.channel);
        this.stopListening();
        await reporter.run(event)
            .then(() => {
                getLogger().logDebug`IndexProgressReporter: progress for total ${event.total} complete`;
            })
            .catch((err: Error) => {
                getLogger().logDebug`IndexProgressReporter: progress for total ${event.total} cancelled: ${err}`;
            });
        this.startListening();
    }
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    try {
        getLogger().logInformation`Initializing searchium`;
        const config = vscode.workspace.getConfiguration("searchium");
        const useLegacyServer = config.get<boolean>("useLegacyServer", true);

        if (useLegacyServer) {
            const [proxy, channel, client] = await startLegacyServer(context);
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
            const history = new SearchHistory(context);

            const searchResultsProvider = new SearchResultsProvider(channel);
            context.subscriptions.push(vscode.window.registerTreeDataProvider("searchium-results", searchResultsProvider));
            const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
                { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
            const searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, channel, history);

            const controlsProvider = new ControlsProvider(context, context.extensionUri, indexState, history);
            context.subscriptions.push(vscode.window.registerWebviewViewProvider("searchium-controls", controlsProvider));

            context.subscriptions.push(new IndexProgressReporter(channel));
            context.subscriptions.push(new DocumentRegistrationService(context, client));

            // const fileSearchManager = new FileSearchManager(channel);
            const detailsPanelProvider = new DetailsPanelProvider(context, channel);

            context.subscriptions.push(
                vscode.commands.registerCommand("searchium.query", searchManager.onQuery, searchManager),
                vscode.commands.registerCommand('searchium.nextResult', searchManager.navigateToNextResult, searchManager),
                vscode.commands.registerCommand('searchium.previousResult', searchManager.navigateToPreviousResult, searchManager),

                // Not working very well with chromium server
                // vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager),

                vscode.commands.registerCommand("searchium.openDetails", detailsPanelProvider.openDetails, detailsPanelProvider),

                // todo: rename commands 
                vscode.commands.registerCommand("searchium.focusSearch", controlsProvider.onJumpToSearchInput, controlsProvider),
                vscode.commands.registerCommand("searchium.newSearch", controlsProvider.onNewSearch, controlsProvider),
                vscode.commands.registerCommand("searchium.clearHistory", controlsProvider.onClearHistory, controlsProvider),
                vscode.commands.registerTextEditorCommand("searchium.searchCurrentToken", controlsProvider.onSearchCurrentToken, controlsProvider),

                vscode.commands.registerCommand("searchium.toggleCaseSensitivity", controlsProvider.onToggleCaseSensitivity, controlsProvider),
                vscode.commands.registerCommand("searchium.toggleWholeWord", controlsProvider.onToggleWholeWord, controlsProvider),
                vscode.commands.registerCommand("searchium.toggleRegex", controlsProvider.onToggleRegex, controlsProvider),
                vscode.commands.registerCommand("searchium.previousQuery", controlsProvider.onPreviousQuery, controlsProvider),
                vscode.commands.registerCommand("searchium.nextQuery", controlsProvider.onNextQuery, controlsProvider),
            );
        }
        else {
            const [process, client] = await startServer(context);
            context.subscriptions.push(process);
            context.subscriptions.push(new DocumentRegistrationService(context, client));
        }
    } catch (err) {
        getLogger().logError`Unexpected error initializing extension: ${err}`;
    }
}

export function deactivate(): void {
    getLogger().logInformation`Deactivating searchium extension`;
}
