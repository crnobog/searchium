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
import { IndexClient } from 'index/indexInterface';
import * as pb2 from 'gen/searchium/v2/searchium';

async function drainIndexProgress(
    progress: vscode.Progress<{ increment: number, message: string }>,
    event: pb2.IndexProgressUpdate,
    iterator: AsyncIterator<pb2.IndexProgressUpdate>
): Promise<void> {
    const total = event.total;
    let completed = event.completed;
    if (completed === total) { return; }
    getLogger().logInformation`Starting index progress ${completed} ... ${total}`;
    progress.report({ increment: completed / total * 100, message: event.message });
    for (; ;) {
        const i = await iterator.next();
        if (i.done) { return; }
        const e = i.value;
        if (e.total !== total) {
            getLogger().logInformation`Indexing aborted ${e.total} not equal ${total}`;
            return;
        }
        getLogger().logInformation`Continuing index progress ${completed} ... ${total}`;
        progress.report({ increment: (e.completed - completed) / e.total * 100, message: e.message });
        completed = e.completed;
        if (e.completed === e.total) {
            getLogger().logInformation`Indexing complete ${e.completed}`;
            return;
        }
    }
}

class IndexProgressReporter implements vscode.Disposable {
    abort = false;
    constructor(private client: IndexClient) {
        this.startListening();
    }

    public dispose(): void {
        this.abort = true;
    }

    private async startListening(): Promise<void> {
        const iterable = this.client.getIndexProgress();
        const iterator = iterable[Symbol.asyncIterator]();
        getLogger().logInformation`IndexProgressReporter.startListening`;
        for (; ;) {
            const i = await iterator.next();
            if (i.done || this.abort) { break; }
            const e: pb2.IndexProgressUpdate = i.value;
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

            context.subscriptions.push(new IndexProgressReporter(client));
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
