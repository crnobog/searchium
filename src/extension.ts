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
import { DetailsPanelProvider } from './detailsPanel';
import { SearchHistory } from './history';

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
        getLogger().logInformation`Listening on port ${port}`;

        let hostExePath = path.join(context.extensionPath, "bin", "VSChromium.Host.exe");
        let serverExePath = path.join(context.extensionPath, "bin", "VSChromium.Server.exe");
        this.proc = child_process.spawn(hostExePath, [serverExePath, `${port}`], { detached: true });

        const c = await pendingSocket;
        getLogger().logInformation`Received connection from search server`;
        c.on('end', () => {
            getLogger().logInformation`Search server disconnected`;
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
            getLogger().logInformation`Handshaking successful!`;
        }
        catch (err: any) {
            vscode.window.showErrorMessage("Error handshaking with search server process.");
            getLogger().logError`Handshake error: ${err}`;
        }
        return channel;
    }

    private onError(err: Error) {
        getLogger().logError`Connection error: ${err}`;
    }
}

class IndexProgressInstance {
    total: number;
    completed: number;
    listener?: (e: ipcEvents.TypedEvent) => void;

    constructor(event: ipcEvents.ProgressReportEvent, private channel: IpcChannel) {
        this.total = event.total;
        this.completed = event.completed;
    }

    public async run(event: ipcEvents.ProgressReportEvent) {
        let options: vscode.ProgressOptions = {
            location: vscode.ProgressLocation.Notification,
            cancellable: false,
            title: "Searchium Indexing"
        };
        await vscode.window.withProgress(options, this.progress.bind(this));
    }

    private async progress(progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken): Promise<void> {
        progress.report({ increment: this.completed / this.total * 100, message: "" });
        if (this.completed === this.total) {
            return Promise.resolve();
        }
        return new Promise<void>((resolve, reject) => {
            this.listener = (e) => {
                if (e.eventType !== 'progressReport') { return; }

                // If total changed, start a new progress window
                if (e.total !== this.total) {
                    reject();
                    return;
                }

                progress.report({ increment: (this.completed - e.completed) / e.total * 100, message: e.displayText });
                this.completed = e.completed;
                if (e.completed === e.total) {
                    resolve();
                }
            };
            this.channel.on('event', this.listener!);
        })
            .then(() => {
                this.channel.off('event', this.listener!);
            })
            .catch((err) => {
                this.channel.off('event', this.listener!);
            });
    }
}

class IndexProgressReporter {
    listener: (event: ipcEvents.TypedEvent) => void;
    constructor(private channel: IpcChannel) {
        this.listener = this.onIpcEvent.bind(this);
        this.startListening();
    }

    private onIpcEvent(event: ipcEvents.TypedEvent) {
        if (event.eventType === 'progressReport') {
            this.startProgress(event);
        }
    }

    private startListening() {
        getLogger().logDebug`IndexProgressReporter: start listening`;
        this.channel.on('event', this.listener);
    }
    private stopListening() {
        getLogger().logDebug`IndexProgressReporter: stop listening`;
        this.channel.off('event', this.listener);
    }

    private async startProgress(event: ipcEvents.ProgressReportEvent) {
        getLogger().logDebug`IndexProgressReporter: starting progress report total ${event.total}`;
        let reporter = new IndexProgressInstance(event, this.channel);
        this.stopListening();
        await reporter.run(event)
            .then(() => {
                getLogger().logDebug`IndexProgressReporter: progress for total ${event.total} complete`;
            })
            .catch((err: any) => {
                getLogger().logDebug`IndexProgressReporter: progress for total ${event.total} cancelled`;
            });
        this.startListening();
    }
}

export async function activate(context: vscode.ExtensionContext) {
    try {
        getLogger().logInformation`Initializing searchium`;
        const proxy = new ServerProxy();
        let channel = await proxy.startServer(context);

        channel.on('response', (r) => {
            switch (r.responseType) {
                default:
                    getLogger().logDebug`response: ${r}`;
                case 'searchCode':
                    break;
            }
        });
        channel.on('event', (e) => getLogger().logDebug`event: ${e}`);

        let indexState = new IndexState(channel);
        const history = new SearchHistory(context);

        const searchResultsProvider = new SearchResultsProvider(channel);
        context.subscriptions.push(vscode.window.registerTreeDataProvider("searchium-results", searchResultsProvider));
        const searchResultsTreeView = vscode.window.createTreeView('searchium-results',
            { treeDataProvider: searchResultsProvider, canSelectMany: false, dragAndDropController: undefined, showCollapseAll: true });
        const searchManager = new SearchManager(searchResultsProvider, searchResultsTreeView, channel, history);

        const controlsProvider = new ControlsProvider(context, context.extensionUri, indexState, history);
        context.subscriptions.push(vscode.window.registerWebviewViewProvider("searchium-controls", controlsProvider));

        let reporter = new IndexProgressReporter(channel);
        context.subscriptions.push(new DocumentRegistrationService(context, channel));

        // const fileSearchManager = new FileSearchManager(channel);
        const detailsPanelProvider = new DetailsPanelProvider(context, channel);

        context.subscriptions.push(
            vscode.commands.registerCommand("searchium.query", searchManager.onQuery, searchManager),
            vscode.commands.registerCommand('searchium.nextResult', searchManager.navigateToNextResult, searchManager),
            vscode.commands.registerCommand('searchium.previousResult', searchManager.navigateToPreviousResult, searchManager),

            // Not working very well with chromium server
            // vscode.commands.registerCommand("searchium.searchFilePaths", fileSearchManager.onSearchFilePaths, fileSearchManager),

            vscode.commands.registerCommand("searchium.openDetails", detailsPanelProvider.openDetails, detailsPanelProvider),

            // todo: rename commands 
            vscode.commands.registerCommand("searchium.focusSearch", controlsProvider.onJumpToSearchInput, controlsProvider),
            vscode.commands.registerCommand("searchium.newSearch", controlsProvider.onNewSearch, controlsProvider),
            vscode.commands.registerCommand("searchium.clearHistory", controlsProvider.onClearHistory, controlsProvider),
            vscode.commands.registerTextEditorCommand("searchium.searchCurrentToken", controlsProvider.onSearchCurrentToken, controlsProvider),

            vscode.commands.registerCommand("searchium.toggleCaseSensitivity", controlsProvider.onToggleCaseSensitivity, controlsProvider),
            vscode.commands.registerCommand("searchium.toggleWholeWord", controlsProvider.onToggleWholeWord, controlsProvider),
            vscode.commands.registerCommand("searchium.toggleRegex", controlsProvider.onToggleRegex, controlsProvider),
            vscode.commands.registerCommand("searchium.previousQuery", controlsProvider.onPreviousQuery, controlsProvider),
            vscode.commands.registerCommand("searchium.nextQuery", controlsProvider.onNextQuery, controlsProvider),
        );
    } catch (err: any) {

    }
}

export function deactivate() { }
