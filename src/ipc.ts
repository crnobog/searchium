import * as searchium_pb from './gen/searchium';

interface MessageBase {
    requestId: bigint;
    protocol: string;
    data: MessageData;
}

export type Response = MessageBase;

export interface StringData {
    dataType: "stringData";
    text?: string;
}

export interface ErrorResponse {
    dataType: "errorResponse";
    message?: string;
    fullTypeName?: string;
    stackTrace?: string;
    innerError?: ErrorResponse;
}

export interface TypedMessage {
    dataType: "typedMessage";
    className: string;
}

export type MessageData = StringData | ErrorResponse | TypedMessage;

export abstract class TypedRequest implements TypedMessage {
    dataType: "typedMessage";
    className: string;

    constructor() {
        this.dataType = "typedMessage";
        this.className = "Unknown";
    }
    public abstract toProto(): searchium_pb.TypedRequest;
}