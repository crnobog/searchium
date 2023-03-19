import * as searchium_pb from './gen/searchium_pb';

interface MessageBase {
    requestId: BigInt;
    protocol: string;
    data: MessageData;
}

interface Request extends MessageBase {
    runOnSequentialQueue?: boolean;
}
export interface Response extends MessageBase {
}
export interface Event extends MessageBase {
}

export interface StringData {
    type: "stringData";
    text?: string;
}

export interface ErrorResponse {
    type: "errorResponse";
    message?: string;
    fullTypeName?: string;
    stackTrace?: string;
    innerError?: ErrorResponse;
}

export interface TypedMessage {
    type: "typedMessage"
}

export type MessageData = StringData | ErrorResponse | TypedMessage;