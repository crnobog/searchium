import { PlainMessage } from '@bufbuild/protobuf';
import * as searchium_pb from './gen/searchium_pb';

interface MessageBase {
    requestId: bigint;
    protocol: string;
    data: MessageData;
}

interface Request extends MessageBase {
    runOnSequentialQueue?: boolean;
}
export interface Response extends MessageBase {
}

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
    public abstract toProto(): PlainMessage<searchium_pb.TypedRequest>;
}