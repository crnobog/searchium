import * as vscode from 'vscode';
import { DocumentRegistrationService } from './documentRegistrationService';
import { getLogger } from './logger';
import { SearchResultsProvider, SearchManager } from './search';
import { ControlsProvider } from './controlsProvider';
import { IndexState } from './indexState';
import { DetailsPanelProvider } from './detailsPanel';
import { SearchHistory } from './history';
import { startServer } from './index/indexServerProcess';
import { FileSearchManager } from 'fileSearch';


export async function activate(context: vscode.ExtensionContext): Promise<void> {
    try {
        getLogger().logInformation`Initializing searchium`;
        const history = new SearchHistory(context);
        // TODO: Progress bar/status bar update for this? 
        const [process, client] = await startServer(context);
        const fileSearchManager = new FileSearchManager(client);
        const searchResultsProvider = new SearchResultsProvider(client);
        const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
            { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
        const searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, client, history);
        const indexState = new IndexState(client);
        const controlsProvider = new ControlsProvider(context, context.extensionUri, history, indexState);
        const detailsPanelProvider = new DetailsPanelProvider(context, client);
        context.subscriptions.push(
            process,
            new DocumentRegistrationService(context, client),
            fileSearchManager,
            vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager)
        );

        const toMbString = (value: bigint): string =>
            `${(Number(value / 1024n) / 1024.0).toFixed(2)} MB`;
        setInterval(async () => {
            const info = await client.getProcessInfo();
            getLogger().logInformation`Index server physical memory: ${toMbString(info.physicalMemory)} Virtual memory: ${toMbString(info.virtualMemory)}`;
        }, 10 * 1000);

        context.subscriptions.push(
            vscode.window.registerWebviewViewProvider("searchium-controls", controlsProvider, { webviewOptions: { retainContextWhenHidden: true } }),

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
        getLogger().logInformation`Searchium initialized`;
    } catch (err) {
        getLogger().logError`Unexpected error initializing extension: ${err}`;
    } 
}

export function deactivate(): void {
    getLogger().logInformation`Deactivating searchium extension`;
}
