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

export class RegisterFileRequest extends TypedRequest {
    fileName: string;

    constructor(fileName: string) {
        super();
        this.fileName = fileName;
    }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "registerFileRequest",
                value: { fileName: this.fileName }
            }
        };
    }
}

export class UnregisterFileRequest extends TypedRequest {
    fileName: string;

    constructor(fileName: string) {
        super();
        this.fileName = fileName;
    }

    public toProto(): PlainMessage<searchium_pb.TypedRequest> {
        return {
            subtype: {
                case: "unregisterFileRequest",
                value: { fileName: this.fileName }
            }
        };
    }
}

export class TypedResponse implements TypedMessage {
    dataType: 'typedMessage';
    className: string;

    constructor(className: string) {
        this.dataType = 'typedMessage';
        this.className = className;
    }
}