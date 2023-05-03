import * as vscode from "vscode";
import * as child_process from "child_process";
import * as path from "path";
import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import { ChannelCredentials } from "@grpc/grpc-js";
import * as pb from "gen/searchium/v2/searchium";
import { ISearchiumServiceClient, SearchiumServiceClient } from 'gen/searchium/v2/searchium.client';
import { getLogger } from 'logger';
import { DuplexStreamingMethod, IndexClient } from "./indexInterface";

class IndexServerProcess implements vscode.Disposable {
    constructor(
        private proc: child_process.ChildProcessWithoutNullStreams | undefined,
        private transport: GrpcTransport,
    ) {
    }

    public dispose(): void {
        this.proc?.kill();
        this.transport.close();
    }
};

class IndexServerClient implements IndexClient {
    constructor(private client: ISearchiumServiceClient) { }
    public registerWorkspaceFolder(request: pb.FolderRegisterRequest): AsyncIterable<pb.IndexUpdate> {
        return this.client.registerFolder(request).responses;
    }
    public async unregisterWorkspaceFolder(request: pb.FolderUnregisterRequest): Promise<void> {
        await this.client.unregisterFolder(request).response;
    }
    public searchFilePaths(): DuplexStreamingMethod<pb.FilePathSearchRequest, pb.FilePathSearchResponse> {
        const res = this.client.searchFilePaths();
        return {
            send: async (message: pb.FilePathSearchRequest) => {
                await res.requests.send(message);
            },
            complete: async () => {
                await res.requests.complete();
            },
            results: res.responses
        };
    }
    public searchFileContents(request: pb.FileContentsSearchRequest): Promise<pb.FileContentsSearchResponse> {
        return this.client.searchFileContents(request).response;
    }
    public getFileExtracts(filePath: string, extracts: pb.Span[], maxLen: number): Promise<pb.FileExtractsResponse> {
        return this.client.getFileExtracts({ filePath, spans: extracts, maxExtractLength: maxLen }).response;
    }
    public async getProcessInfo(): Promise<pb.ProcessInfoResponse> {
        return await this.client.getProcessInfo({}).response;
    }
    public async getDatabaseDetails(): Promise<pb.DatabaseDetailsResponse> {
        return await this.client.getDatabaseDetails({}).response;
    }
}

export async function startServer(context: vscode.ExtensionContext): Promise<[IndexServerProcess, IndexClient]> {
    let host = vscode.workspace.getConfiguration("searchium").get<string>("debugIndexHost");
    let childProc: child_process.ChildProcessWithoutNullStreams | undefined;
    if (!host) {
        const serverExePath = path.join(context.extensionPath, "bin", "searchium-server.exe");
        const proc = child_process.spawn(serverExePath, [], { detached: true });
        childProc = proc;
        host = await new Promise<string>((resolve, _reject) => {
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
    }
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

    return [new IndexServerProcess(childProc, transport), new IndexServerClient(client)];
}