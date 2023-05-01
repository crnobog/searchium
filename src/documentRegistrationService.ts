import { nextTick } from "process";
import * as vscode from "vscode";
import { getLogger } from "./logger";
import { IndexClient } from "./index/indexInterface";
import { IpcChannel } from "ipcChannel";
import * as ipcRequests from "ipcRequests";
import { Timestamp } from "gen/google/protobuf/timestamp";

export class DocumentRegistrationService implements vscode.Disposable {
    constructor(
        private context: vscode.ExtensionContext,
        private channel: IpcChannel | undefined,
        private client: IndexClient | undefined
    ) {
        nextTick(() => this.run());
    }

    public dispose(): void {
        return;
    }

    public async run(): Promise<void> {
        if (vscode.workspace.workspaceFolders) {
            for (const folder of vscode.workspace.workspaceFolders) {
                if (folder.uri.scheme === 'file') {
                    this.register(folder);
                }
            }
        }
        vscode.workspace.onDidChangeWorkspaceFolders(this.onWorkspaceFoldersChanged, this);
    }

    private onWorkspaceFoldersChanged(event: vscode.WorkspaceFoldersChangeEvent): void {
        for (const added of event.added) {
            if (added.uri.scheme === 'file') {
                this.register(added);
            }
        }
        for (const removed of event.removed) {
            if (removed.uri.scheme === 'file') {
                this.unregister(removed);
            }
        }
    }

    private register(folder: vscode.WorkspaceFolder): void {
        // TODO: Hook configuration changes 
        if (this.channel) {
            this.channel.sendSequentialRequest(new ipcRequests.RegisterFileRequest(folder.uri.fsPath));
        }

        if (this.client) {
            const filesExclude: { [_: string]: boolean } = vscode.workspace.getConfiguration("files", folder).get("exclude", {});
            const searchExclude: { [_: string]: boolean } = vscode.workspace.getConfiguration("search", folder).get("exclude", {});
            const request = {
                path: folder.uri.fsPath,
                ignoreFileGlobs: Object.entries(filesExclude).filter(v => v[1]).map(v => v[0]),
                ignoreSearchGlobs: Object.entries(searchExclude).filter(v => v[1]).map(v => v[0]),
            };
            const events = this.client.registerWorkspaceFolder(request);
            vscode.window.withProgress(
                { location: vscode.ProgressLocation.Notification, cancellable: false, title: "Searchium" },
                async (progress, _token) => {
                    let lastCount = 0;
                    for await (const event of events) {
                        switch (event.type.oneofKind) {
                            case 'filesystemScanStart': {
                                getLogger().logInformation`Filesystem scan started at ${Timestamp.toDate(event.timestamp ?? Timestamp.now())}`;
                                progress.report({ message: `Scanning filesystem.` });
                                break;
                            }
                            case 'filesystemScanEnd': {
                                getLogger().logInformation`Filesystem scan ended at ${Timestamp.toDate(event.timestamp ?? Timestamp.now())}`;
                                progress.report({ message: `Finished scanning filesystem.` });
                                break;
                            }
                            case 'fileContentsLoaded': {
                                getLogger().logInformation`File load ended at ${Timestamp.toDate(event.timestamp ?? Timestamp.now())}`;
                                const count = event.type.fileContentsLoaded.count;
                                const total = event.type.fileContentsLoaded.total;
                                const increment = (count - lastCount) / total * 100;
                                lastCount = count;
                                progress.report({ message: `${event.type.fileContentsLoaded.path}`, increment });
                                break;
                            }
                        }
                    }
                }
            );
        }
    }
    private unregister(folder: vscode.WorkspaceFolder): void {
        if (this.channel) {
            this.channel.sendSequentialRequest(new ipcRequests.UnregisterFileRequest(folder.uri.fsPath));
        }

        if (this.client) {
            this.client.unregisterWorkspaceFolder({ path: folder.uri.fsPath })
                .then(() => getLogger().logDebug`Completed unregister for ${folder.name}`)
                .catch((e) => getLogger().logError`Error unregistering file ${folder.name}: ${e}`);
        }
    }
}