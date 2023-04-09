import { nextTick } from "process";
import * as vscode from "vscode";
import * as ipcRequests from './ipcRequests';
import { IpcChannel } from "./ipcChannel";
import { getLogger } from "./logger";

export class DocumentRegistrationService implements vscode.Disposable {
    channel: IpcChannel;
    context: vscode.ExtensionContext;

    constructor(context: vscode.ExtensionContext, channel: IpcChannel) {
        this.context = context;
        this.channel = channel;

        nextTick(() => this.run());
    }

    public dispose(): void {
        return;
    }

    public async run(): Promise<void> {
        if (vscode.workspace.workspaceFolders) {
            for (const folder of vscode.workspace.workspaceFolders) {
                if (folder.uri.scheme === 'file') {
                    this.registerPath(folder.uri.fsPath);
                }
            }
        }
        vscode.workspace.onDidChangeWorkspaceFolders(this.onWorkspaceFoldersChanged, this);
        for (const editor of vscode.window.visibleTextEditors) {
            const uri = editor.document.uri;
            if (uri.scheme !== 'file') {
                continue;
            }
            this.registerPath(uri.fsPath);
        }
    }

    private onWorkspaceFoldersChanged(event: vscode.WorkspaceFoldersChangeEvent): void {
        for (const added of event.added) {
            if (added.uri.scheme === 'file') {
                this.registerPath(added.uri.fsPath);
            }
        }
        for (const removed of event.removed) {
            if (removed.uri.scheme === 'file') {
                this.unregisterPath(removed.uri.fsPath);
            }
        }
    }

    private registerPath(path: string): void {
        this.channel.sendSequentialRequest(new ipcRequests.RegisterFileRequest(path))
            .then(() => getLogger().logDebug`Completed register for ${path}`)
            .catch((e) => getLogger().logError`Error registering file ${path}: ${e}`);
    }
    // TODO: Refcount for multiple openings of documents? 
    private unregisterPath(path: string): void {
        this.channel.sendSequentialRequest(new ipcRequests.UnregisterFileRequest(path))
            .then(() => getLogger().logDebug`Completed unregister for ${path}`)
            .catch((e) => getLogger().logError`Error unregistering file ${path}: ${e}`);
    }
}