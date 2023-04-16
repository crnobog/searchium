// TODO: Use protobuf-generated types here or not?
import * as pb from "gen/searchium/v2/searchium";

// TODO: Settings for workspace folders:
//  files.exclude
//  search.exclude
//  search.useIgnoreFiles
//  search.useParentIgnoreFiles
//  search.useGlobalIgnoreFiles
export interface IndexClient {
    registerWorkspaceFolder(request: pb.FolderRegisterRequest): AsyncIterable<pb.IndexUpdate>;
    unregisterWorkspaceFolder(request: pb.FolderUnregisterRequest): Promise<void>;
}