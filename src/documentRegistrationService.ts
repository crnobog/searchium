import { nextTick } from "process";
import * as vscode from "vscode";
import * as ipc from './ipc';
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

    public dispose() {
    }

    public async run() {
        for (const editor of vscode.window.visibleTextEditors) {
            const uri = editor.document.uri;
            if (uri.scheme !== 'file') {
                continue;
            }
            this.registerPath(uri.fsPath);
        }
        this.context.subscriptions.push(
            vscode.workspace.onDidOpenTextDocument(this.onTextDocumentOpened, this),
            vscode.workspace.onDidCloseTextDocument(this.onTextDocumentClosed, this)
        );
    }

    private onTextDocumentOpened(doc: vscode.TextDocument) {
        if (doc.uri.scheme === 'file') { this.registerPath(doc.uri.fsPath); }
    }
    private onTextDocumentClosed(doc: vscode.TextDocument) {
        if (doc.uri.scheme === 'file') { this.unregisterPath(doc.uri.fsPath); }
    }

    private registerPath(path: string) {
        this.channel.sendSequentialRequest(new ipcRequests.RegisterFileRequest(path))
            .then(() => getLogger().log`Completed register for ${path}`)
            .catch((e) => getLogger().log`Error registering file ${path}: ${e}`);
    }
    // TODO: Refcount for multiple openings of documents? 
    private unregisterPath(path: string) {
        this.channel.sendSequentialRequest(new ipcRequests.UnregisterFileRequest(path))
            .then(() => getLogger().log`Completed unregister for ${path}`)
            .catch((e) => getLogger().log`Error unregistering file ${path}: ${e}`);
    }
}