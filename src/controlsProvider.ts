import * as vscode from 'vscode';
import { getLogger } from './logger';
import { SearchOptions } from './search';
import { IndexState } from './indexState';
import { GetDatabaseStatisticsResponse } from './ipcResponses';
import { Search } from '@microsoft/fast-foundation';
import { IndexingServerStatus } from './gen/searchium_pb';

export function getNonce() {
    let text = "";
    const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}

export function getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
    return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
}

const DEFAULT_SEARCH_OPTIONS: SearchOptions = {
    query: "",
    pathFilter: "",
    matchCase: false,
    wholeWord: false,
    regex: false
};

function html(strings: TemplateStringsArray, ...insertions: any[]) {
    let s = "";
    for (let i = 0; i < insertions.length; ++i) {
        s += strings[i];
        try {
            let insertion = insertions[i];
            if (insertion instanceof Object && insertion.toString === Object.prototype.toString) {
                s += JSON.stringify(insertion, (key, value) => {
                    if (typeof value === 'bigint') { return value.toString(); }
                    else { return value; }
                });
            }
            else {
                s += `${insertion}`;
            }
        } catch {
            s += "LOG_ERROR";
        }
    }
    s += strings[strings.length - 1];
    return s;
}

export class ControlsProvider implements vscode.WebviewViewProvider {
    webview?: vscode.Webview;
    databaseStats?: GetDatabaseStatisticsResponse;

    constructor(
        private readonly context: vscode.ExtensionContext,
        private readonly extensionUri: vscode.Uri,
        private readonly indexState: IndexState
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

        this.webview = webviewView.webview;
        webviewView.webview.options = {
            enableScripts: true
        };
        webviewView.onDidDispose(() => this.onViewDisposed(webviewView));

        webviewView.webview.html = this.getWebViewContent(webviewView.webview, this.extensionUri, this.getControlsState());
        webviewView.webview.onDidReceiveMessage(this.onMessage, this);
    }

    public async onNewSearch() {
        let query = await vscode.window.showInputBox({
            title: "Searchium",
            prompt: "Search term",
        });
        if (!query) { return; }
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
            this.setQueryString(query);
            // TODO: Should upate ui state to match these settings?
            vscode.commands.executeCommand("searchium.query", <SearchOptions>{
                query,
                matchCase: true,
                pathFilter: "", // TODO: Should inherit path filter or not?
                regex: false,
                wholeWord: editor.selection.isEmpty // Only do whole-word search if we selected the token ourselves 
            });
        }
    }
    public onEnableCaseSensitive() {
        // todo: update ui 
        this.updateControlsState({ matchCase: true });
    }
    public onDisableCaseSensitive() {
        // todo: update ui
        this.updateControlsState({ matchCase: false });
    }
    private onViewDisposed(webviewView: vscode.WebviewView) {
        this.webview = undefined;
    }

    private setQueryString(query: string) {
        // Set state in case webview is not created yet 
        this.updateControlsState({ query });
        this.webview?.postMessage({
            type: "setQuery",
            value: query
        });
    }

    private sendStatsToWebview() {
        if (this.databaseStats && this.webview) {
            if (this.databaseStats.projectCount === 0) {
                this.webview.postMessage({
                    type: 'nostatus'
                });
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
            this.webview.postMessage({
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

    private onMessage(msg: any) {
        getLogger().log`webview message: ${msg}`;
        switch (msg.command) {
            case 'ready':
                this.sendStatsToWebview();
                break;
            case 'execute':
                let state = this.getControlsState();
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
                break;
            case 'setWholeWord':
                this.updateControlsState({ wholeWord: msg.value });
                break;
            case 'setRegex':
                this.updateControlsState({ regex: msg.value });
                break;
        }
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri, initialState: SearchOptions): string {
        const webviewUri = getUri(webview, extensionUri, ["out", "webview.js"]);
        const codiconsUri = getUri(webview, extensionUri, ['node_modules', '@vscode/codicons', 'dist', 'codicon.css']);
        const stylesheetUri = getUri(webview, extensionUri, ["out", "style.css"]);

        const commandUri = vscode.Uri.parse("command:searchium.query");

        const nonce = getNonce();
        let content = html`
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
        <vscode-text-field class="search-input" id="query-input" value="${initialState.query}">
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