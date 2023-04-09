import * as searchium_pb from './gen/searchium';

export class RegisterFileRequest {
    constructor(public fileName: string) { }
    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: "registerFileRequest",
                registerFileRequest: { fileName: this.fileName }
            }
        };
    }
}

export class UnregisterFileRequest {
    constructor(public fileName: string) { }

    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: "unregisterFileRequest",
                unregisterFileRequest: { fileName: this.fileName }
            }
        };
    }
}

export class SearchCodeRequest {
    constructor(
        public searchParams: searchium_pb.SearchParams) {
    }

    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: "searchCodeRequest",
                searchCodeRequest: { searchParams: this.searchParams }
            }
        };
    }
}

export class GetFileExtractsRequest {
    constructor(
        public fileName: string,
        public positions: searchium_pb.FilePositionSpan[],
        public maxExtractLength: number) {
    }

    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: "getFileExtractsRequest",
                getFileExtractsRequest: {
                    fileName: this.fileName,
                    positions: this.positions,
                    maxExtractLength: this.maxExtractLength,
                }
            }
        };
    }
}

export class GetDatabaseStatisticsRequest {
    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: 'getDatabaseStatisticsRequest',
                getDatabaseStatisticsRequest: {
                    forceGarbageCollection: false
                }
            }
        };
    }
}

export class SearchFilePathsRequest {
    constructor(
        private searchParams: searchium_pb.SearchParams
    ) { }

    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: 'searchFilePathsRequest',
                searchFilePathsRequest: {
                    searchParams: this.searchParams
                }
            }
        };
    }
}

export class GetDatabaseDetailsRequest {
    constructor(
        private maxFilesByExtensionDetails: number,
        private maxLargeFilesDetailsCount: number
    ) {
    }

    public toProto(): searchium_pb.TypedRequest {
        return {
            subtype: {
                oneofKind: 'getDatabaseDetailsRequest',
                getDatabaseDetailsRequest: {
                    maxFilesByExtensionDetailsCount: this.maxFilesByExtensionDetails,
                    maxLargeFilesDetailsCount: this.maxLargeFilesDetailsCount,
                }
            }
        };
    }
}

export type Request = RegisterFileRequest
    | UnregisterFileRequest
    | SearchCodeRequest
    | GetFileExtractsRequest
    | GetDatabaseStatisticsRequest
    | SearchFilePathsRequest
    | GetDatabaseDetailsRequest;