import * as child_process from 'child_process';
import { AddressInfo, createServer, Server, Socket } from 'net';
import * as path from 'path';
import { TypedEmitter } from 'tiny-typed-emitter';
import * as vscode from 'vscode';
import { DocumentRegistrationService } from './documentRegistrationService';
import * as ipcEvents from './ipcEvents';
import * as ipc from './ipc';
import { IpcChannel } from './ipcChannel';
import { getLogger } from './logger';
import * as searchium_pb from './gen/searchium_pb';
import { SearchResultsProvider, SearchManager } from './search';
import { ControlsProvider } from './controlsProvider';
import { IndexState } from './indexState';
import { FileSearchManager } from './fileSearch';

class ServerProxy implements vscode.Disposable {
    listener?: Server;
    proc?: child_process.ChildProcessWithoutNullStreams;

    constructor() {
    }

    public dispose() {
        this.listener?.close();
        this.proc?.kill();
    }

    public async startServer(context: vscode.ExtensionContext): Promise<IpcChannel> {
        let pendingSocket = new Promise<Socket>((resolve, reject) => {
            this.listener = createServer((c) => resolve(c));
        });
        this.listener?.on('error', this.onError.bind(this));
        context.subscriptions.push(this);

        const portTask = new Promise<number>((resolve, reject) => {
            this.listener?.on('error', (e) => { reject(e); });
            this.listener?.on('listening', () => {
                const port = (this.listener?.address() as AddressInfo).port;
                resolve(port);
            });
        });
        this.listener?.listen();
        const port = await portTask;
        getLogger().log`Listening on port ${port}`;

        let hostExePath = path.join(context.extensionPath, "bin", "VSChromium.Host.exe");
        let serverExePath = path.join(context.extensionPath, "bin", "VSChromium.Server.exe");
        this.proc = child_process.spawn(hostExePath, [serverExePath, `${port}`], { detached: true });

        const c = await pendingSocket;
        getLogger().log`Received connection from search server`;
        c.on('end', () => {
            getLogger().log`Search server disconnected`;
        });

        const channel = new IpcChannel(c);
        context.subscriptions.push(channel);
        const handshake = new Promise<void>((resolve, reject) => {
            channel.once('raw', (r: searchium_pb.IpcMessage) => {
                if (!r.data) { return reject("Empty initial response"); }
                if (r.data.subtype.case !== 'ipcStringData') {
                    return reject(new Error("Expected initial response to contain string data"));
                }
                let message = r.data.subtype.value.text;
                if (message !== 'Hello!') {
                    return reject(new Error("Expected initial response string to be 'Hello!'"));
                }
                resolve();
            });
        });

        process.nextTick(async () => {
            await channel.drainDispatch();
        });

        try {
            await handshake;
            getLogger().log`Handshaking successful!`;
        }
        catch (err: any) {
            vscode.window.showErrorMessage("Error handshaking with search server process.");
            getLogger().log`Handshake error: ${err}`;
        }
        return channel;
    }

    private onError(err: Error) {
        getLogger().log`Connection error: ${err}`;
    }
}

class IndexProgressReporter {
    constructor(private channel: IpcChannel) {
        this.channel.once('event', this.onIpcEvent.bind(this));
    }

    public onIpcEvent(event: ipcEvents.TypedEvent) {
        if (event.eventType === 'progressReport') {
            this.startProgress(event);
        }
    }

    private startProgress(event: ipcEvents.ProgressReportEvent) {
        let options: vscode.ProgressOptions = {
            location: vscode.ProgressLocation.Notification,
            cancellable: false,
            title: event.displayText
        };
        let task = (progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken) => {
            let total = event.total;
            let completed = 0;
            progress.report({ increment: event.completed / event.total * 100, message: "" });
            completed = event.completed;
            return new Promise<void>((resolve, reject) => {
                this.channel.on('event', (e) => {
                    if (e.eventType !== 'progressReport') { return; }

                    // If total changed, start a new progress window
                    if (event.total !== total) { reject(); this.startProgress(event); return; }

                    progress.report({ increment: (completed - event.completed) / event.total * 100, message: event.displayText });
                    completed = event.completed;
                    if (event.completed === event.total) {
                        resolve();
                    }
                });
            });
        };
        vscode.window.withProgress(options, task);
    }
}

export async function activate(context: vscode.ExtensionContext) {
    getLogger().log`Initializing searchium`;

    try {
        const proxy = new ServerProxy();
        let channel = await proxy.startServer(context);

        channel.on('response', (r) => {
            switch (r.responseType) {
                default:
                    getLogger().log`response: ${r}`;
                case 'searchCode':
                    break;
            }
        });
        channel.on('event', (e) => getLogger().log`event: ${e}`);

        let indexState = new IndexState(channel);
        const searchResultsProvider = new SearchResultsProvider(channel);

        context.subscriptions.push(vscode.window.registerTreeDataProvider("searchium-results", searchResultsProvider));
        const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
            { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
        const searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, channel);

        let controlsProvider = new ControlsProvider(context, context.extensionUri, indexState);
        context.subscriptions.push(vscode.window.registerWebviewViewProvider("searchium-controls", controlsProvider));

        let reporter = new IndexProgressReporter(channel);
        context.subscriptions.push(new DocumentRegistrationService(context, channel));

        // const fileSearchManager = new FileSearchManager(channel);

        context.subscriptions.push(
            vscode.commands.registerCommand("searchium.query", searchManager.onQuery, searchManager),
            vscode.commands.registerCommand('searchium.nextResult', searchManager.navigateToNextResult, searchManager),
            vscode.commands.registerCommand('searchium.previousResult', searchManager.navigateToPreviousResult, searchManager),

            // Not working very well with chromium server
            // vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager),

            // todo: rename commands 
            vscode.commands.registerCommand("searchium.focusSearch", controlsProvider.onJumpToSearchInput, controlsProvider),
            vscode.commands.registerCommand("searchium.newSearch", controlsProvider.onNewSearch, controlsProvider),
            vscode.commands.registerTextEditorCommand("searchium.searchCurrentToken", controlsProvider.onSearchCurrentToken, controlsProvider),

            vscode.commands.registerCommand("searchium.toggleCaseSensitivity", controlsProvider.onToggleCaseSensitivity, controlsProvider),
            vscode.commands.registerCommand("searchium.toggleWholeWord", controlsProvider.onToggleWholeWord, controlsProvider),
            vscode.commands.registerCommand("searchium.toggleRegex", controlsProvider.onToggleRegex, controlsProvider)
        );
    } catch (err: any) {

    }
}

export function deactivate() { }
