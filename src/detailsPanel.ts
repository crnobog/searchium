import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import { getUri, getNonce } from './webviewUtils';

export class DetailsPanelProvider {
    constructor(
        private context: vscode.ExtensionContext,
        private channel: IpcChannel) {

    }

    public openDetails() {
        const panel = vscode.window.createWebviewPanel('searchium-details', "Searchium Index Details", vscode.ViewColumn.Active, {
            enableScripts: true
        });
        panel.webview.html = this.getWebViewContent(panel.webview, this.context.extensionUri);
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri): string {
        const webviewUri = getUri(webview, extensionUri, ["out", "webview", "controls.js"]);
        const codiconsUri = getUri(webview, extensionUri, ['node_modules', '@vscode/codicons', 'dist', 'codicon.css']);
        const stylesheetUri = getUri(webview, extensionUri, ["out", "webview", "style.css"]);

        const commandUri = vscode.Uri.parse("command:searchium.query");

        const nonce = getNonce();
        let content =/*html*/ `
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
    <section id="progress-container">
        <vscode-progress-ring class="progress-ring"></vscode-progress-ring>
        <div class=progress-text>Fetching details...</div>
    </section>
    <section id="details-container">
    </section>
    <script type="module" nonce="${nonce}" src="${webviewUri}"></script>
</body>

</html>
        `;
        return content;
    }
}