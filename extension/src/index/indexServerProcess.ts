import * as vscode from "vscode";
import * as child_process from "child_process";
import * as path from "path";
import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import { ChannelCredentials } from "@grpc/grpc-js";
import { ISearchiumServiceClient, SearchiumServiceClient } from 'gen/searchium/service.client';
import { getLogger } from 'logger';
import { DuplexStreamingMethod, IndexClient, DatabaseDetails, DatabaseDetailsRoot, IndexStatus } from "./indexInterface";
import { FileContentsSearchRequest } from "gen/searchium/file_contents_search_request";
import { FileContentsSearchResponse } from "gen/searchium/file_contents_search_response";
import { FileContentsSpan } from "gen/searchium/file_contents_span";
import { FileExtractsResponse } from "gen/searchium/file_extracts_response";
import { FilePathSearchRequest } from "gen/searchium/file_path_search_request";
import { FilePathSearchResponse } from "gen/searchium/file_path_search_response";
import { FolderRegisterRequest } from "gen/searchium/folder_register_request";
import { FolderUnregisterRequest } from "gen/searchium/folder_unregister_request";
import { IndexUpdate } from "gen/searchium/index_update";
import { IndexState } from "gen/searchium/status_response";
import { ProcessInfoResponse } from "gen/searchium/process_info_response";

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
    public registerWorkspaceFolder(request: FolderRegisterRequest): AsyncIterable<IndexUpdate> {
        return this.client.registerFolder(request).responses;
    }
    public async unregisterWorkspaceFolder(request: FolderUnregisterRequest): Promise<void> {
        await this.client.unregisterFolder(request).response;
    }
    public searchFilePaths(): DuplexStreamingMethod<FilePathSearchRequest, FilePathSearchResponse> {
        const res = this.client.searchFilePaths();
        return {
            send: async (message: FilePathSearchRequest) => {
                await res.requests.send(message);
            },
            complete: async () => {
                await res.requests.complete();
            },
            results: res.responses
        };
    }
    public searchFileContents(request: FileContentsSearchRequest): Promise<FileContentsSearchResponse> {
        return this.client.searchFileContents(request).response;
    }
    public getFileExtracts(filePath: string, extracts: FileContentsSpan[], maxLen: number): Promise<FileExtractsResponse> {
        return this.client.getFileExtracts({ filePath, matchSpans: extracts, maxExtractLength: maxLen }).response;
    }
    public async getProcessInfo(): Promise<ProcessInfoResponse> {
        return await this.client.getProcessInfo({}).response;
    }
    public async getDatabaseDetails(): Promise<DatabaseDetails> {
        const response = await this.client.getDatabaseDetails({}).response;
        return {
            roots: response.roots.map((p: DatabaseDetailsRoot): DatabaseDetailsRoot => {
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
                case IndexState.UNAVAILABLE:
                    state = "Unavailable";
                    break;
                case IndexState.READY:
                    state = "Ready";
                    break;
                case IndexState.INDEXING:
                    state = "Indexing";
                    break;
                case IndexState.PAUSED:
                    state = "Paused";
                    break;
            }
            yield { state, memUsage: r.memUsage, numSearchableFiles: r.numSearchableFiles };
        }
    }
}

export async function startServer(context: vscode.ExtensionContext): Promise<[IndexServerProcess, IndexClient]> {
    let host = process.env["SEARCHIUM_DEBUG_HOST"];
    let childProc: child_process.ChildProcessWithoutNullStreams | undefined;
    if (host) {
        getLogger().logInformation`Connecting to existing debug server on ${host}`;
    }
    else {
        const debugExe = Boolean(process.env["SEARCHIUM_DEBUG"] ?? false);
        const serverExePath = path.join(context.extensionPath, debugExe ? "bin-debug" : "bin", "searchium-server.exe");
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
        getLogger().logInformation`Started server ${serverExePath} and it is listening at ${host}`;
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