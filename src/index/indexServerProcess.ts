import * as vscode from "vscode";
import * as child_process from "child_process";
import * as path from "path";
import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import { ChannelCredentials } from "@grpc/grpc-js";
import * as pb from "gen/searchium/v2/searchium";
import { ISearchiumServiceClient, SearchiumServiceClient } from 'gen/searchium/v2/searchium.client';
import { getLogger } from 'logger';
import { DuplexStreamingMethod, IndexClient, DatabaseDetails, DatabaseDetailsRoot, IndexStatus } from "./indexInterface";

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
    public async getDatabaseDetails(): Promise<DatabaseDetails> {
        const response = await this.client.getDatabaseDetails({}).response;
        return {
            roots: response.roots.map((p: pb.DatabaseDetailsRoot): DatabaseDetailsRoot => {
                return {
                    rootPath: p.rootPath,
                    numFilesScanned: p.numFilesScanned,
                    numDirectoriesScanned: p.numDirectoriesScanned,
                    numSearchableFiles: p.numSearchableFiles,
                    searchableFilesBytes: p.searchableFilesBytes,
                    numBinaryFiles: p.numBinaryFiles,
                    binaryFilesBytes: p.binaryFilesBytes,
                    searchableFilesByExtension: p.searchableFilesByExtension,
                    binaryFilesByExtension: p.binaryFilesByExtension,
                    largeSearchableFiles: p.largeSearchableFiles,
                    largeBinaryFiles: p.largeBinaryFiles,
                };
            })
        };
    }
    public async* getStatus(): AsyncIterable<IndexStatus> {
        for await (const r of this.client.getStatus({}).responses) {
            let state: IndexStatus["state"];
            switch (r.state) {
                case pb.IndexState.UNAVAILABLE:
                    state = "Unavailable";
                    break;
                case pb.IndexState.READY:
                    state = "Ready";
                    break;
                case pb.IndexState.INDEXING:
                    state = "Indexing";
                    break;
                case pb.IndexState.PAUSED:
                    state = "Paused";
                    break;
            }
            yield { state, memUsage: r.memUsage, numSearchableFiles: r.numSearchableFiles };
        }
    }
}

export async function startServer(context: vscode.ExtensionContext): Promise<[IndexServerProcess, IndexClient]> {
    let host = vscode.workspace.getConfiguration("searchium").get<string>("debugIndexHost");
    let childProc: child_process.ChildProcessWithoutNullStreams | undefined;
    if (host) {
        getLogger().logInformation`Connecting to existing debug server on ${host}`;
    }
    else {
        const serverExePath = path.join(context.extensionPath, "bin", "searchium-server.exe");
        const proc = child_process.spawn(serverExePath, [], { detached: true });
        if (!proc) {
            throw new Error("Failed to create server process");
        }
        childProc = proc;
        // Wait for the first line of output from the new process telling us its host address/port
        host = await new Promise<string>((resolve, reject) => {
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

            let err = "";
            const errListener = (s: string): void => {
                err += s;
                const i = err.indexOf('\n');
                if (i !== -1) {
                    proc.stderr.off('data', errListener);
                    reject(msg.substring(0, i));
                }
            };
            proc.stderr.on('data', errListener);
            // TODO: More error conditions?
        });
        getLogger().logInformation`server at ${host}`;
    }
    const transport = new GrpcTransport({
        host,
        channelCredentials: ChannelCredentials.createInsecure(),
    });

    const client = new SearchiumServiceClient(transport);
    await client.hello({ id: "node" })
        .then((resp) => {
            const r = resp.response;
            getLogger().logInformation`hello grpc response ${JSON.stringify(r)}`;
        })
        .catch((err: Error) => {
            getLogger().logError`Connection error ${err}`;
            throw err;
        });

    return [new IndexServerProcess(childProc, transport), new IndexServerClient(client)];
}