import * as vscode from 'vscode';
import { getLogger } from './logger';

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
    constructor(private readonly extensionUri: vscode.Uri) { }
    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext<unknown>,
        token: vscode.CancellationToken): void | Thenable<void> {

        webviewView.webview.options = {
            enableScripts: true
        };
        webviewView.onDidDispose(this.onViewDisposed, this);

        webviewView.webview.html = this.getWebViewContent(webviewView.webview, this.extensionUri);

        webviewView.webview.onDidReceiveMessage(this.onMessage, this);
    }

    private onViewDisposed() {

    }

    private onMessage(msg: any) {
        getLogger().log`webview message: ${msg}`;
        switch (msg.command) {
            case 'query':
                vscode.commands.executeCommand("searchium.query", msg.text);
        }
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri): string {
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
    <title>Hello World!</title>
</head>

<body>
    <section style="display:grid" id="query-form">
        <vscode-text-field class="search-input" id="query-input">
            <span slot="start" class="codicon codicon-search"></span>
            
            <input type="checkbox" id="check-case-sensitive" 
                   slot="end" class="search-inline-check monaco-custom-toggle" />
            <label for="check-case-sensitive" slot="end" 
                   class="codicon codicon-case-sensitive search-inline-check-label">
            </label>

            <input type="checkbox" id="check-whole-word" slot="end" 
                   class="search-inline-check" />
            <label for="check-whole-word" slot="end" 
                   class="codicon codicon-whole-word search-inline-check-label">
            </label>

            <input type="checkbox" id="check-regex" slot="end" 
                   class="search-inline-check" />
            <label for="check-regex" slot="end" 
                   class="codicon codicon-regex search-inline-check-label">
            </label>
        </vscode-text-field>
        <vscode-text-field class="search-input">
            <span slot="start" class="codicon codicon-folder" />
        </vscode-text-field>
    </section>
    <script type="module" nonce="${nonce}" src="${webviewUri}"></script>
</body>

</html>
        `;
        return content;
    }
}