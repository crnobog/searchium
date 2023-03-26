import * as vscode from 'vscode';

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

        webviewView.webview.html = this.getWebViewContent(webviewView.webview, this.extensionUri);
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri): string {
        const webviewUri = getUri(webview, extensionUri, ["out", "webview.js"]);
        const codiconsUri = getUri(webview, extensionUri, ['node_modules', '@vscode/codicons', 'dist', 'codicon.css']);
        const stylesheetUri = getUri(webview, extensionUri, ["out", "style.css"]);
        const nonce = getNonce();
        return html`
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
    <vscode-text-field class="search-input">
        <span slot="start" class="codicon codicon-search" />
    </vscode-text-field>
    <br>
    <vscode-text-field class="search-input">
        <span slot="start" class="codicon codicon-folder" />
    </vscode-text-field>
    <script type="module" nonce="${nonce}" src="${webviewUri}"></script>
</body>

</html>
        `;
    }
}