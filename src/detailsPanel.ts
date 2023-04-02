import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import { getUri, getNonce } from './webviewUtils';
import * as ipcRequests from './ipcRequests';
import * as ipcResponses from './ipcResponses';
import * as FromWebview from './shared/fromDetailsWebview';
import * as ToWebview from './shared/toDetailsWebview';
import * as searchium_pb from "./gen/searchium_pb";
import { PlainMessage } from "@bufbuild/protobuf";

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
        panel.webview.onDidReceiveMessage(this.onReceiveMessage.bind(this));
        this.webview = panel.webview;
        panel.webview.html = this.getWebViewContent(panel.webview, this.context.extensionUri);

    }

    private async onReceiveMessage(msg: FromWebview.Message) {
        let response = await this.channel.sendRequest(new ipcRequests.GetDatabaseDetailsRequest(
            100, 100
        )) as ipcResponses.GetDatabaseDetailsResponse;

        this.sendMessage({
            type: "details",
            projects: response.projects.map((p): ToWebview.ProjectDetails => {
                let toMbString = (value: bigint) =>
                    `${(Number(value / 1024n) / 1024.0).toFixed(2)} MB`;
                let mapByExtension = (details: PlainMessage<searchium_pb.FileByExtensionDetails>) => {
                    return {
                        extension: details.fileExtension,
                        count: details.fileCount.toLocaleString(),
                        size: toMbString(details.fileByteLength)
                    };
                };
                let mapLarge = (details: PlainMessage<searchium_pb.LargeFileDetails>) => {
                    return {
                        path: details.relativePath,
                        size: toMbString(details.byteLength)
                    };
                };
                let mapConfig = (details: PlainMessage<searchium_pb.ProjectConfigurationSectionDetails> | undefined) => {
                    return {
                        path: details?.containingFilePath ?? "",
                        name: details?.name ?? "",
                        contents: details?.contents ?? "",
                    };
                };
                return {
                    rootPath: p.rootPath,
                    numFiles: (p.directoryDetails?.fileCount ?? 0).toLocaleString(),
                    numDirectories: (p.directoryDetails?.directoryCount ?? 0).toLocaleString(),
                    numSearchableFiles: (p.directoryDetails?.searchableFilesCount ?? 0).toLocaleString(),
                    searchableFilesMB: Number((p.directoryDetails?.searchableFilesByteLength ?? 0n) / 1024n) / 1024.0,
                    numBinaryFiles: (p.directoryDetails?.binaryFilesCount ?? 0).toLocaleString(),
                    binaryFilesMB: Number((p.directoryDetails?.binaryFilesByteLength ?? 0n) / 1024n) / 1024.0,
                    searchableByExtension: p.directoryDetails?.searchableFilesByExtensionDetails.map(mapByExtension) ?? [],
                    largeFiles: p.directoryDetails?.largeSearchableFileDetails.map(mapLarge) ?? [],
                    binaryByExtension: p.directoryDetails?.binaryFilesByExtensionDetails.map(mapByExtension) ?? [],
                    largeBinaries: p.directoryDetails?.largeBinaryFilesDetails.map(mapLarge) ?? [],
                    ignorePaths: mapConfig(p.projectConfigurationDetails?.ignorePathsSection),
                    ignoreFiles: mapConfig(p.projectConfigurationDetails?.ignoreSearchableFilesSection),
                    includeFiles: mapConfig(p.projectConfigurationDetails?.includeSearchableFilesSection),
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
            <div class="details-label">Number of files scanned:</div>
            <span class="details-value" id="details-num-files"></span>
            <div class="details-label">Number of directories scanned:</div>
            <span class="details-value" id="details-num-directories"></span>
            <div class="details-label">Number of searchable files:</div>
            <span class="details-value" id="details-num-searchable-files"></span>
            <div class="details-label">Total size of searchable files:</div>
            <span class="details-value" id="details-size-searchable-files"></span>
            <div class="details-label">Number of binary files scanned:</div>
            <span class="details-value" id="details-num-binary-files"></span>
            <div class="details-label">Total size of binary files:</div>
            <span class="details-value" id="details-size-binary-files"></span>
        </section>
        <vscode-panels class="details-panels">
            <vscode-panel-tab id="tab-extensions">Types</vscode-panel-tab>
            <vscode-panel-tab id="tab-large-files">Large Files</vscode-panel-tab>
            <vscode-panel-tab id="tab-binary-extensions">Binary Types</vscode-panel-tab>
            <vscode-panel-tab id="tab-large-binaries">Large Binaries</vscode-panel-tab>
            <vscode-panel-tab id="tab-configuration">Configuration</vscode-panel-tab>
            
            <vscode-panel-view id="view-extensions">
                <vscode-data-grid id="grid-extensions"
                grid-template-columns="1fr 1fr 1fr"
                generate-header="sticky"
                >
                </vscode-data-grid>
            </vscode-panel-view>
            <vscode-panel-view id="view-large-files">
                <vscode-data-grid id="grid-large-files"
                grid-template-columns="4fr 1fr"
                generate-header="sticky"
                >
                </vscode-data-grid>
            </vscode-panel-view>
            <vscode-panel-view id="view-binary-extensions">
                <vscode-data-grid id="grid-binary-extensions"
                grid-template-columns="1fr 1fr 1fr"
                generate-header="sticky"
                >
                </vscode-data-grid>
            </vscode-panel-view>
            <vscode-panel-view id="view-large-binaries">
                <vscode-data-grid id="grid-large-binaries"
                grid-template-columns="4fr 1fr"
                generate-header="sticky"
                >
                </vscode-data-grid>
            </vscode-panel-view>
            <vscode-panel-view id="view-configuration">
                <section class="config-details">
                    <section class="ignore-paths">
                        Ignore Paths
                        <vscode-divider role="separator"></vscode-divider>
                        <vscode-text-field class="config-file-path" id="ignore-paths-path" readonly>Config File Path</vscode-text-field>
                        <vscode-text-field class="config-section-name" id="ignore-paths-name" readonly>Section Name</vscode-text-field>
                        <vscode-text-area class="config-contents" id="ignore-paths-contents"
                            rows=10
                            readonly>
                            Contents
                        </vscode-text-area>
                    </section>
                    <section class="ignore-files">
                        Ignore Files
                        <vscode-divider role="separator"></vscode-divider>
                        <vscode-text-field class="config-file-path" id="ignore-files-path" readonly>Config File Path</vscode-text-field>
                        <vscode-text-field class="config-section-name" id="ignore-files-name" readonly>Section Name</vscode-text-field>
                        <vscode-text-area class="config-contents" id="ignore-files-contents"
                            rows=10
                            readonly>
                            Contents
                        </vscode-text-area>
                    </section>
                    <section class="include-files">
                        Include Files
                        <vscode-divider role="separator"></vscode-divider>
                        <vscode-text-field class="config-file-path" id="include-files-path" readonly>Config File Path</vscode-text-field>
                        <vscode-text-field class="config-section-name" id="include-files-name" readonly>Section Name</vscode-text-field>
                        <vscode-text-area class="config-contents" id="include-files-contents"
                            rows=10
                            readonly>
                            Contents
                        </vscode-text-area>
                    </section>
                </section>
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