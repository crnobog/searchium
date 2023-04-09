import { nextTick } from "process";
import * as vscode from "vscode";
import { getLogger } from "./logger";
import { IndexClient } from "./index/indexInterface";

export class DocumentRegistrationService implements vscode.Disposable {
    constructor(private context: vscode.ExtensionContext, private client: IndexClient) {
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
        this.client.registerWorkspaceFolder(path)
            .then(() => getLogger().logDebug`Completed register for ${path}`)
            .catch((e) => getLogger().logError`Error registering file ${path}: ${e}`);
    }
    private unregisterPath(path: string): void {
        this.client.unregisterWorkspaceFolder(path)
            .then(() => getLogger().logDebug`Completed unregister for ${path}`)
            .catch((e) => getLogger().logError`Error unregistering file ${path}: ${e}`);
    }
}