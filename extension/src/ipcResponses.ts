import * as searchium_legacy from './gen/searchium';

export interface ResponseBase {
    requestId: bigint;
}

export interface DoneResponse extends ResponseBase {
    responseType: "done";
    info: string;
}

export interface SearchCodeResponse extends ResponseBase {
    responseType: "searchCode";
    searchResults: searchium_legacy.FileSystemEntry; // should always be a DirectoryEntry
    hitCount: bigint;
    searchedFileCount: bigint;
    totalFileCount: bigint;
}

export interface GetFileExtractsResponse extends ResponseBase {
    responseType: "getFileExtracts";
    fileName: string;
    fileExtracts: searchium_legacy.FileExtract[];
}

export interface GetDatabaseStatisticsResponse extends ResponseBase {
    responseType: "getDatabaseStatistics";
    projectCount: number;
    fileCount: bigint;
    searchableFileCount: bigint;
    serverNativeMemoryUsage: bigint;
    serverGcMemoryUsage: bigint;
    lastIndexUpdatedUtc: Date;
    serverStatus: searchium_legacy.IndexingServerStatus;
}

export interface SearchFilePathsResponse extends ResponseBase {
    responseType: "searchFilePaths";
    searchResult: searchium_legacy.FileSystemEntry;
    hitCount: bigint;
    totalCount: bigint;
}

export interface GetDatabaseDetailsResponse extends ResponseBase {
    responseType: "getDatabaseDetails";
    projects: searchium_legacy.ProjectDetails[]
}

export type Response = DoneResponse
    | SearchCodeResponse
    | GetFileExtractsResponse
    | GetDatabaseStatisticsResponse
    | SearchFilePathsResponse
    | GetDatabaseDetailsResponse;