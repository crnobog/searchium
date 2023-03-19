import * as child_process from 'child_process';
import { AddressInfo, createServer, Server, Socket } from 'net';
import * as path from 'path';
import { TypedEmitter } from 'tiny-typed-emitter';
import * as vscode from 'vscode';
import * as ipc from './ipc';
import { IpcChannel } from './ipcChannel';
import { getLogger } from './logger';

class ServerProxy {
    server: Server;
    extensionContext: vscode.ExtensionContext;
    channel?: IpcChannel;

    constructor(context: vscode.ExtensionContext) {
        this.extensionContext = context;
        this.server = createServer(this.onConnectionReceived.bind(this));
        this.server.on('error', this.onError.bind(this));
        this.server.on('listening', this.onListening.bind(this));
        this.server.listen();
    }

    private onListening() {
        const port = (this.server.address() as AddressInfo).port;
        getLogger().log`Listening on port ${port}`;
        // TODO: Check necessity of intermediate host process and 'break away from job' - does that ensure server can live longer than extension runner? 
        let hostExePath = path.join(this.extensionContext.extensionPath, "bin", "VSChromium.Host.exe");
        let serverExePath = path.join(this.extensionContext.extensionPath, "bin", "VSChromium.Server.exe");
        child_process.spawn(hostExePath, [serverExePath, `${port}`]);
    }

    private onError(err: Error) {
        getLogger().log`Connection error: ${err}`;
    }

    private async onConnectionReceived(c: Socket) {
        getLogger().log`Received connection from search server`;
        c.on('end', () => {
            getLogger().log`Search server disconnected`;
        });

        const channel = new IpcChannel(c);
        this.channel = channel;
        const handshake = new Promise<void>((resolve, reject) => {
            channel.once('raw', (r: ipc.Response) => {
                if (r.data.type !== 'stringData') {
                    return reject(new Error("Expected initial response to contain string data"));
                }
                let message = r.data as ipc.StringData;
                if (message.text !== 'Hello!') {
                    return reject(new Error("Expected initial response string to be 'Hello!'"));
                }
                resolve();
            });
        });

        // TODO: Wait for a response message which should match HelloWorldProtocol.cs
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
    }
}

export function activate(context: vscode.ExtensionContext) {
    getLogger().log`Initializing searchium`;
    context.subscriptions.push(vscode.commands.registerCommand('searchium.helloWorld', () => {
        vscode.window.showInformationMessage('Hello searchium!');
    }));

    try {
        const proxy = new ServerProxy(context);
    } catch (err: any) {

    }
}

export function deactivate() { }
