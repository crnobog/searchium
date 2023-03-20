import { PlainMessage } from '@bufbuild/protobuf';
import * as searchium_pb from './gen/searchium_pb';

export interface ResponseBase {
    requestId: bigint;
}

export interface DoneResponse extends ResponseBase {
    responseType: "done";
    info: string;
}

export interface SearchCodeResponse extends ResponseBase {
    responseType: "searchCode";
    searchResults: PlainMessage<searchium_pb.FileSystemEntry>; // should always be a DirectoryEntry
    hitCount: bigint;
    searchedFileCount: bigint;
    totalFileCount: bigint;
}

export interface GetFileExtractsResponse extends ResponseBase {
    responseType: "getFileExtracts";
    fileName: string;
    fileExtracts: PlainMessage<searchium_pb.FileExtract>[];
}

export type Response = DoneResponse
    | SearchCodeResponse
    | GetFileExtractsResponse;