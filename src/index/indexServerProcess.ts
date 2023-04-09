import * as vscode from "vscode";
import * as child_process from "child_process";
import * as path from "path";
import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import { ChannelCredentials } from "@grpc/grpc-js";
import { ISearchiumServiceClient, SearchiumServiceClient } from 'gen/searchium/v2/searchium.client';
import { getLogger } from 'logger';
import { IndexClient } from "./indexInterface";

class IndexServerProcess implements vscode.Disposable {
    constructor(
        private proc: child_process.ChildProcessWithoutNullStreams,
        private transport: GrpcTransport,
    ) {
    }

    public dispose(): void {
        this.proc.kill();
        this.transport.close();
    }
};

class IndexServerClient implements IndexClient {
    constructor(private client: ISearchiumServiceClient) { }

    public registerWorkspaceFolder(): Promise<void> {
        throw new Error("Method not implemented.");
    }
    public unregisterWorkspaceFolder(): Promise<void> {
        throw new Error("Method not implemented.");
    }
}

export async function startServer(context: vscode.ExtensionContext): Promise<[IndexServerProcess, IndexClient]> {
    const serverExePath = path.join(context.extensionPath, "bin", "searchium-server.exe");
    const proc = child_process.spawn(serverExePath, [], { detached: true });
    const host: string = await new Promise((resolve, _reject) => {
        let msg = "";
        const listener = (s: string): void => {
            msg += s;
            const i = msg.indexOf('\n');
            if (i !== -1) {
                proc.stdout.off('data', listener);
                resolve(msg.substring(0, i));
            }
        };
        proc.stdout.on('data', listener);
        // TODO: Error conditions
    });
    const transport = new GrpcTransport({
        host,
        channelCredentials: ChannelCredentials.createInsecure(),
    });

    const client = new SearchiumServiceClient(transport);
    await client.hello({ id: "node" })
        .then((resp) => {
            const r = resp.response;
            getLogger().logInformation`grpc response ${JSON.stringify(r)}`;
        })
        .catch((err: Error) => {
            getLogger().logError`Error ${err}`;
        });

    return [new IndexServerProcess(proc, transport), new IndexServerClient(client)];
}