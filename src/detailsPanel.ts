import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import { getUri, getNonce } from './webviewUtils';
import * as ipcRequests from './ipcRequests';
import * as ipcResponses from './ipcResponses';
import * as FromWebview from './shared/fromDetailsWebview';
import * as ToWebview from './shared/toDetailsWebview';

export class DetailsPanelProvider {
    webview?: vscode.Webview;
    constructor(
        private context: vscode.ExtensionContext,
        private channel: IpcChannel) {
    }

    public async openDetails() {
        const panel = vscode.window.createWebviewPanel('searchium-details', "Searchium Index Details", vscode.ViewColumn.Active, {
            enableScripts: true
        });
        let ready = new Promise<void>((resolve, reject) => {
            panel.webview.onDidReceiveMessage((msg) => {
                resolve();
            });
        });
        this.webview = panel.webview;
        panel.webview.html = this.getWebViewContent(panel.webview, this.context.extensionUri);

        let response = await this.channel.sendRequest(new ipcRequests.GetDatabaseDetailsRequest(
            100, 100
        )) as ipcResponses.GetDatabaseDetailsResponse;

        await ready;
        this.sendMessage({
            type: "details",
            projects: response.projects.map((p): ToWebview.ProjectDetails => {
                return {
                    rootPath: p.rootPath,
                    numFiles: (p.directoryDetails?.fileCount ?? 0).toLocaleString(),
                    numDirectories: (p.directoryDetails?.directoryCount ?? 0).toLocaleString(),
                    numSearchableFiles: (p.directoryDetails?.searchableFilesCount ?? 0).toLocaleString(),
                    searchableFilesMB: Number((p.directoryDetails?.searchableFilesByteLength ?? 0n) / 1024n) / 1024.0,
                    numBinaryFiles: (p.directoryDetails?.binaryFilesCount ?? 0).toLocaleString(),
                    binaryFilesMB: Number((p.directoryDetails?.binaryFilesByteLength ?? 0n) / 1024n) / 1024.0,
                };
            })
        });
    }

    private sendMessage(msg: ToWebview.Message) {
        this.webview?.postMessage(msg);
    }

    private getWebViewContent(webview: vscode.Webview, extensionUri: vscode.Uri): string {
        const webviewUri = getUri(webview, extensionUri, ["out", "webview", "details.js"]);
        const codiconsUri = getUri(webview, extensionUri, ["out", "webview", 'codicon.css']);
        const stylesheetUri = getUri(webview, extensionUri, ["out", "webview", "style.css"]);

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
    <section id="progress-container" class="progress-container">
        <vscode-progress-ring class="progress-ring"></vscode-progress-ring>
        <div class=progress-text>Fetching details...</div>
    </section>
    <section id="details-container" class="hidden">
        <vscode-dropdown id="projects-dropdown"></vscode-dropdown>
        <section id="project-details" class="project-details">
            <div class="details-element">Number of files scanned:<span id="details-num-files" class="details-element"></span></div>
            <div class="details-element">Number of directories scanned:<span id="details-num-directories" class="details-element"></span></div>
            <div class="details-element">Number of searchable files:<span id="details-num-searchable-files" class="details-element"></span></div>
            <div class="details-element">Total size of searchable files:<span id="details-size-searchable-files" class="details-element"></span></div>
            <div class="details-element">Number of binary files scanned:<span id="details-num-binary-files" class="details-element"></span></div>
            <div class="details-element">Total size of binary files:<span id="details-size-binary-files" class="details-element"></span></div>
        </section>
        <vscode-panels class="details-panels">
            <vscode-panel-tab id="tab-extensions">Types</vscode-panel-tab>
            <vscode-panel-tab id="tab-large-files">Large Files</vscode-panel-tab>
            <vscode-panel-tab id="tab-binary-extensions">Binary Types</vscode-panel-tab>
            <vscode-panel-tab id="tab-large-binaries">Large Binaries</vscode-panel-tab>
            <vscode-panel-tab id="tab-configuration">Configuration</vscode-panel-tab>
            
            <vscode-panel-view id="view-extensions">
            </vscode-panel-view>
            <vscode-panel-view id="view-larger-files">
            </vscode-panel-view>
            <vscode-panel-view id="view-binary-extensions">
            </vscode-panel-view>
            <vscode-panel-view id="view-large-binaries">
            </vscode-panel-view>
            <vscode-panel-view id="view-configuration">
            </vscode-panel-view>
        </vscode-panels>
    </section>
    <script type="module" nonce="${nonce}" src="${webviewUri}"></script>
</body>

</html>
        `;
        return content;
    }
}