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
        const filesExclude: { [_: string]: boolean } = vscode.workspace.getConfiguration("files", folder).get("exclude", {});
        const searchExclude: { [_: string]: boolean } = vscode.workspace.getConfiguration("search", folder).get("exclude", {});
        const request = {
            path: folder.uri.fsPath,
            ignoreFileGlobs: Object.entries(filesExclude).filter(v => v[1]).map(v => v[0]),
            ignoreSearchGlobs: Object.entries(searchExclude).filter(v => v[1]).map(v => v[0]),
        };
        this.client.registerWorkspaceFolder(request)
            .then(() => getLogger().logDebug`Completed register for ${folder.name}`)
            .catch((e) => getLogger().logError`Error registering file ${folder.name}: ${e}`);
    }
    private unregister(folder: vscode.WorkspaceFolder): void {
        this.client.unregisterWorkspaceFolder({ path : folder.uri.fsPath })
            .then(() => getLogger().logDebug`Completed unregister for ${folder.name}`)
            .catch((e) => getLogger().logError`Error unregistering file ${folder.name}: ${e}`);
    }
}