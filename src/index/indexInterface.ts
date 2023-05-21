// TODO: Use protobuf-generated types here or not?
import * as pb from "gen/searchium/v2/searchium";

// TODO: re-export types from pb?

export interface DatabaseDetails {
    roots: DatabaseDetailsRoot[],
}
export interface DatabaseDetailsRoot {
    rootPath: string,
    numFilesScanned: bigint,
    numDirectoriesScanned: bigint,
    numSearchableFiles: bigint,
    searchableFilesBytes: bigint,
    numBinaryFiles: bigint,
    binaryFilesBytes: bigint,
    searchableFilesByExtension: FilesByExtensionDetails[],
    binaryFilesByExtension: FilesByExtensionDetails[],
    largeSearchableFiles: LargeFileDetails[],
    largeBinaryFiles: LargeFileDetails[],
}
export interface FilesByExtensionDetails {
    extension: string,
    count: bigint,
    bytes: bigint,
}
export interface LargeFileDetails {
    path: string,
    bytes: bigint,
}
export interface IndexStatus {
    state: "Unavailable" | "Ready" | "Indexing" | "Paused";
    memUsage: bigint;
    numSearchableFiles: bigint;
}

export interface DuplexStreamingMethod<In, Out> {
    send(message: In): Promise<void>;
    complete(): Promise<void>;
    results: AsyncIterable<Out>;
}

// TODO: Settings for workspace folders:
//  files.exclude
//  search.exclude
//  search.useIgnoreFiles
//  search.useParentIgnoreFiles
//  search.useGlobalIgnoreFiles
export interface IndexClient {
    registerWorkspaceFolder(request: pb.FolderRegisterRequest): AsyncIterable<pb.IndexUpdate>;
    unregisterWorkspaceFolder(request: pb.FolderUnregisterRequest): Promise<void>;
    searchFilePaths(): DuplexStreamingMethod<pb.FilePathSearchRequest, pb.FilePathSearchResponse>;
    searchFileContents(request: pb.FileContentsSearchRequest): Promise<pb.FileContentsSearchResponse>;
    getFileExtracts(filePath: string, extracts: pb.Span[], maxLen: number): Promise<pb.FileExtractsResponse>;
    getProcessInfo(): Promise<pb.ProcessInfoResponse>;
    getDatabaseDetails(): Promise<DatabaseDetails>;
    getStatus(): AsyncIterable<IndexStatus>;
}