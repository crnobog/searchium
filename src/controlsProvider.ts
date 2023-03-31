import * as vscode from 'vscode';
import { getLogger } from './logger';
import { SearchOptions } from './search';
import { IndexState } from './indexState';
import { GetDatabaseStatisticsResponse } from './ipcResponses';
import { IndexingServerStatus } from './gen/searchium_pb';
import * as ToWebView from './shared/toControlsWebview';
import * as FromWebView from './shared/fromControlsWebview';
import { getUri, getNonce } from './webviewUtils';
import { SearchHistory } from './history';

const DEFAULT_SEARCH_OPTIONS: SearchOptions = {
    query: "",
    pathFilter: "",
    matchCase: false,
    wholeWord: false,
    regex: false
};

export class ControlsProvider implements vscode.WebviewViewProvider {
    webview?: vscode.Webview;
    databaseStats?: GetDatabaseStatisticsResponse;

    constructor(
        private readonly context: vscode.ExtensionContext,
        private readonly extensionUri: vscode.Uri,
        private readonly indexState: IndexState,
        private readonly history: SearchHistory
    ) {
        this.indexState.on('updated', (response: GetDatabaseStatisticsResponse) => {
            this.databaseStats = response;
            this.sendStatsToWebview();
        });
    }

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        _resolveContext: vscode.WebviewViewResolveContext<unknown>,
        _token: vscode.CancellationToken): void | Thenable<void> {

        getLogger().logDebug`Starting controls webview`;
        this.webview = webviewView.webview;
        webviewView.webview.options = {
            enableScripts: true
        };
        webviewView.onDidDispose(() => this.onViewDisposed(webviewView));
        webviewView.onDidChangeVisibility(() => this.sendMessage({ type: "options", ... this.getControlsState() }));

        webviewView.webview.html = this.getWebViewContent(webviewView.webview, this.extensionUri, this.getControlsState());
        webviewView.webview.onDidReceiveMessage(this.onMessage, this);
    }

    public async onNewSearch() {
        let query = await vscode.window.showInputBox({
            title: "Searchium",
            prompt: "Search term",
        });
        if (!query) { return; }
        getLogger().logInformation`New search for '${query}'`;
        this.setQueryString(query);
        vscode.commands.executeCommand("searchium.query", {
            ... this.getControlsState(), query
        });
    }
    public async onSearchCurrentToken(editor: vscode.TextEditor, edit: vscode.TextEditorEdit) {
        let range = editor.selection.isEmpty
            ? editor.document.getWordRangeAtPosition(editor.selection.start)
            : editor.selection;
        let query = editor.document.getText(range);
        if (query) {
            let options = {
                matchCase: true,
                pathFilter: "",
                query,
                regex: false,
                wholeWord: editor.selection.isEmpty
            };
            this.updateControlsState(options);
            this.sendMessage({
                type: "options",
                ...options
            });
            getLogger().logDebug`Executing current-token search for '${options.query}'`;
            vscode.commands.executeCommand("searchium.query", options);
        }
    }
    public async onJumpToSearchInput() {
        await vscode.commands.executeCommand("searchium-controls.focus");
        this.sendMessage(<ToWebView.FocusMessage>{ type: "focus" });
    }
    public onToggleCaseSensitivity() {
        let matchCase = !this.getControlsState().matchCase;
        getLogger().logInformation`ToggleCaseSensitive to ${matchCase}`;
        this.updateControlsState({ matchCase });
        this.sendMessage({
            type: "setMatchCase",
            matchCase
        });
    }
    public onToggleWholeWord() {
        let wholeWord = !this.getControlsState().wholeWord;
        getLogger().logInformation`ToggleWholeWord to ${wholeWord}`;
        this.updateControlsState({ wholeWord });
        this.sendMessage({
            type: "setWholeWord",
            wholeWord
        });
    }
    public onToggleRegex() {
        let regex = !this.getControlsState().regex;
        getLogger().logInformation`ToggleRegex to ${regex}`;
        this.updateControlsState({ regex });
        this.sendMessage({
            type: "setRegex",
            regex
        });
    }
    public onPreviousQuery() {
        let query = this.history.prev();
        if (query !== undefined) {
            getLogger().logInformation`QueryPrev: '${query}'`;
            this.setQueryString(query);
            vscode.commands.executeCommand("searchium.query", {
                ... this.getControlsState(), query
            }, "history");
        }
        else {
            getLogger().logInformation`QueryPrev: at oldest item`;
        }
    }
    public onNextQuery() {
        let query = this.history.next();
        if (query !== undefined) {
            getLogger().logInformation`QueryNext: '${query}'`;
            this.setQueryString(query);
            vscode.commands.executeCommand("searchium.query", {
                ... this.getControlsState(), query
            }, "history");
        }
        else {
            getLogger().logInformation`QueryNext: at newest item`;
        }
    }
    private onViewDisposed(webviewView: vscode.WebviewView) {
        getLogger().logDebug`Webview disposed`;
        this.webview = undefined;
    }
    private setQueryString(query: string) {
        // Set state in case webview is not created yet 
        this.updateControlsState({ query });
        this.sendMessage({ type: "setQuery", query });
    }

    private sendStatsToWebview() {
        if (this.databaseStats && this.webview) {
            if (this.databaseStats.projectCount === 0) {
                this.sendMessage(<ToWebView.NoStatusMessage>{ type: "nostatus" });
                return;
            }
            let mem = Number(this.databaseStats.serverNativeMemoryUsage) / 1024.0 / 1024.0;
            let state = "Idle";
            switch (this.databaseStats.serverStatus) {
                case IndexingServerStatus.IDLE:
                    state = 'Idle';
                    break;
                case IndexingServerStatus.BUSY:
                    state = 'Busy';
                    break;
                case IndexingServerStatus.PAUSED:
                    state = 'Paused';
                    break;
                case IndexingServerStatus.YIELD:
                    state = 'Yield';
                    break;
            }
            this.sendMessage(<ToWebView.StatusMessage>{
                type: 'status',
                numFiles: `${this.databaseStats.searchableFileCount.toLocaleString()} files`,
                memory: `${mem.toFixed(1)} MB`,
                state
            });
        }
    }
    private getControlsState(): SearchOptions {
        return this.context.workspaceState.get("SEARCH_OPTIONS", DEFAULT_SEARCH_OPTIONS) as any;
    }

    private updateControlsState(update: Partial<SearchOptions>) {
        return this.context.workspaceState.update("SEARCH_OPTIONS", {
            ... this.getControlsState(),
            ...update
        });
    }

    private sendMessage(msg: ToWebView.Message) {
        if (this.webview) {
            this.webview.postMessage(msg);
        }
        else {
            getLogger().logDebug`Dropping message of type ${msg.type} because there is no live webview`;
        }
    }

    private onMessage(msg: FromWebView.Message) {
        getLogger().logDebug`webview message: ${msg}`;
        switch (msg.command) {
            case 'ready':
                this.sendStatsToWebview();
                break;
            case 'execute':
                let state = this.getControlsState();
                getLogger().logInformation`Executing query from controls: '${msg.query}'`;
                vscode.commands.executeCommand("searchium.query", msg);
                break;
            case 'setQuery':
                this.updateControlsState({ query: msg.text });
                break;
            case 'setFilter':
                this.updateControlsState({ pathFilter: msg.text });
                break;
            case 'setMatchCase':
                this.updateControlsState({ matchCase: msg.value });
                vscode.commands.executeCommand("searchium.query", this.getControlsState());
                break;
            case 'setWholeWord':
                this.updateControlsState({ wholeWord: msg.value });
                vscode.commands.executeCommand("searchium.query", this.getControlsState());
                break;
            case 'setRegex':
                this.updateControlsState({ regex: msg.value });
                vscode.commands.executeCommand("searchium.query", this.getControlsState());
                break;
        }
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri, initialState: SearchOptions): string {
        const webviewUri = getUri(webview, extensionUri, ["out", "webview", "controls.js"]);
        const codiconsUri = getUri(webview, extensionUri, ['node_modules', '@vscode/codicons', 'dist', 'codicon.css']);
        const stylesheetUri = getUri(webview, extensionUri, ["out", "webview", "style.css"]);

        const nonce = getNonce();
        let content = /*html*/ `
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="${codiconsUri}" rel="stylesheet" />
    <link href="${stylesheetUri}" rel="stylesheet" />
    <title>Searchium</title>
</head>

<body>
    <section id="query-form">
        <vscode-text-field class="search-input" id="query-input" value="${initialState.query}" autofocus="true">
            <span slot="start" class="codicon codicon-search"></span>

            <input type="checkbox" id="check-case-sensitive" slot="end" class="search-inline-check monaco-custom-toggle"
                ${initialState.matchCase ? "checked" : "" } />
            <label for="check-case-sensitive" slot="end" class="codicon codicon-case-sensitive search-inline-check-label">
            </label>
 
            <input type="checkbox" id="check-whole-word" slot="end" ${initialState.wholeWord ? "checked" : ""}
                class="search-inline-check" />
            <label for="check-whole-word" slot="end" class="codicon codicon-whole-word search-inline-check-label">
            </label>

            <input type="checkbox" id="check-regex" slot="end" ${initialState.regex ? "checked" : "" } class="search-inline-check" />
            <label for="check-regex" slot="end" class="codicon codicon-regex search-inline-check-label">
            </label>
        </vscode-text-field>
        <vscode-text-field class="search-input" id="filter-input" value="${initialState.pathFilter}">
            <span slot="start" class="codicon codicon-folder" />
        </vscode-text-field>
        <div class="control-row">
             <span class="db-state">
                <vscode-tag class="db-needed" id="tag-num-files"></vscode-tag>
                <vscode-tag class="db-needed" id="tag-memory-usage"></vscode-tag>
                <vscode-tag class="db-needed" id="tag-index-state"></vscode-tag>
                <vscode-tag  id="tag-no-db">No DB</vscode-tag>
            </span>
            <vscode-button appearance="primary" id="query-execute">Search</vscode-button>
        </div>
    </section>
    <script type="module" nonce="${nonce}" src="${webviewUri}"></script>
</body>

</html>
        `;
        return content;
    }
}