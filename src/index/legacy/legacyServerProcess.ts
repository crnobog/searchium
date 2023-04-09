import * as vscode from "vscode";
import * as child_process from 'child_process';
import * as path from 'path';
import { AddressInfo, createServer, Server, Socket } from 'net';
import * as searchium_pb from "../../gen/searchium";
import * as ipcRequests from "../../ipcRequests";
import { IpcChannel } from "../../ipcChannel";
import { IndexClient } from "../indexInterface";
import { getLogger } from "../../logger";

export async function startLegacyServer(context: vscode.ExtensionContext): Promise<[vscode.Disposable, IpcChannel, IndexClient]> {
    const proxy = new ServerProxy();
    const channel = await proxy.startServer(context);

    return [proxy, channel, new ChannelClient(channel)];
}

class ChannelClient implements IndexClient {
    constructor(private channel: IpcChannel) { }

    public registerWorkspaceFolder(path: string): Promise<void> {
        return this.channel.sendSequentialRequest(new ipcRequests.RegisterFileRequest(path));
    }

    public unregisterWorkspaceFolder(path: string): Promise<void> {
        return this.channel.sendSequentialRequest(new ipcRequests.UnregisterFileRequest(path));
    }
}

class ServerProxy implements vscode.Disposable {
    listener?: Server;
    proc?: child_process.ChildProcessWithoutNullStreams;

    public dispose(): void {
        this.listener?.close();
        this.proc?.kill();
    }

    public async startServer(context: vscode.ExtensionContext): Promise<IpcChannel> {
        const pendingSocket = new Promise<Socket>((resolve, reject) => {
            try {
                this.listener = createServer((c) => resolve(c));
            } catch (error) {
                reject(error);
            }
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

        const hostExePath = path.join(context.extensionPath, "bin", "VSChromium.Host.exe");
        const serverExePath = path.join(context.extensionPath, "bin", "VSChromium.Server.exe");
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
                if (r.data.subtype.oneofKind !== 'ipcStringData') {
                    return reject(new Error("Expected initial response to contain string data"));
                }
                const message = r.data.subtype.ipcStringData.text;
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
        catch (err) {
            vscode.window.showErrorMessage("Error handshaking with search server process.");
            getLogger().logError`Handshake error: ${err}`;
        }
        return channel;
    }

    private onError(err: Error): void {
        getLogger().logError`Connection error: ${err}`;
    }
}