import * as searchium_pb from './gen/searchium_pb';

export interface ResponseBase {
    requestId: bigint;
}

export interface DoneResponse extends ResponseBase {
    responseType: "done";
    info: string;
}

export type Response = DoneResponse;