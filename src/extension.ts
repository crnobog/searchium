import * as child_process from 'child_process';
import { AddressInfo, createServer, Server, Socket } from 'net';
import * as path from 'path';
import { TypedEmitter } from 'tiny-typed-emitter';
import * as vscode from 'vscode';
import { DocumentRegistrationService } from './documentRegistrationService';
import * as ipc from './ipc';
import { IpcChannel } from './ipcChannel';
import { getLogger } from './logger';
import * as searchium_pb from './gen/searchium_pb';

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

export async function activate(context: vscode.ExtensionContext) {
    getLogger().log`Initializing searchium`;

    try {
        const proxy = new ServerProxy();
        let channel = await proxy.startServer(context);

        channel.on('event', (e) => getLogger().log`event: ${e}`);
        channel.on('response', (r) => getLogger().log`response: ${r}`);

        context.subscriptions.push(new DocumentRegistrationService(context, channel));
    } catch (err: any) {

    }
}

export function deactivate() { }
