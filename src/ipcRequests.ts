import * as searchium_pb from './gen/searchium_pb';
import { PlainMessage } from '@bufbuild/protobuf';

export class RegisterFileRequest {
    constructor(public fileName: string) { }
    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "registerFileRequest",
                value: { fileName: this.fileName }
            }
        };
    }
}

export class UnregisterFileRequest {
    constructor(public fileName: string) { }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "unregisterFileRequest",
                value: { fileName: this.fileName }
            }
        };
    }
}

export class SearchCodeRequest {
    constructor(
        public searchParams: PlainMessage<searchium_pb.SearchParams>) {
    }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "searchCodeRequest",
                value: { searchParams: this.searchParams }
            }
        };
    }
}

export class GetFileExtractsRequest {
    constructor(
        public fileName: string,
        public positions: PlainMessage<searchium_pb.FilePositionSpan>[],
        public maxExtractLength: number) {
    }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "getFileExtractsRequest",
                value: {
                    fileName: this.fileName,
                    positions: this.positions,
                    maxExtractLength: this.maxExtractLength,
                }
            }
        };
    }
}

export class GetDatabaseStatisticsRequest {
    constructor() { }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: 'getDatabaseStatisticsRequest',
                value: {
                    forceGarbageCollection: false
                }
            }
        }
    }
}

export type Request = RegisterFileRequest
    | UnregisterFileRequest
    | SearchCodeRequest
    | GetFileExtractsRequest
    | GetDatabaseStatisticsRequest;