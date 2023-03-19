import { Socket } from "net";
import { TypedEmitter } from "tiny-typed-emitter";
import { protoDelimited } from "@bufbuild/protobuf";
import * as ipc from './ipc';
import * as searchium_pb from './gen/searchium_pb';

interface IpcChannelEvents {
    'raw': (raw: ipc.Response | ipc.Event) => void;
    'response': (r: ipc.Response) => void;
    'event': (e: ipc.Event) => void;
    // 'fatalError' : (e : Error) => void;
}

export class IpcChannel extends TypedEmitter<IpcChannelEvents> {
    socket: Socket;

    constructor(socket: Socket) {
        super();
        this.socket = socket;
    }

    public async drainDispatch() {
        for await (const rawResponse of protoDelimited.decStream(searchium_pb.IpcMessage, this.socket)) {
            try {
                this.dispatchResponse(rawResponse);
            }
            catch {

            }
        }
    }

    private dispatchResponse(msg: searchium_pb.IpcMessage) {
        let response: searchium_pb.IpcResponse;
        switch (msg.requestResponse.case) {
            case undefined: throw new Error("Received unknown ipc message type");
            case "request": throw new Error("Received unexpected ipc request");
            case "response": response = msg.requestResponse.value as searchium_pb.IpcResponse;
        }

        if (response.ipcEvent) {
            let e: ipc.Event = {
                requestId: msg.requestId,
                protocol: msg.protocol,
                data: this.translateMessageData(msg.data)
            };
            this.emit('raw', e);
            this.emit('event', e);
        }
        else {
            let r: ipc.Response = {
                requestId: msg.requestId,
                protocol: msg.protocol,
                data: this.translateMessageData(msg.data)
            };
            this.emit('raw', r);
            this.emit('response', r);
        }
    }

    private translateMessageData(data: searchium_pb.IpcMessageData | undefined): ipc.MessageData {
        if (!data) {
            throw new Error("Missing message data");
        }
        switch (data.subtype.case) {
            case undefined: throw new Error("Missing subtytpe for ipc message data");
            case 'ipcStringData': {
                return {
                    type: 'stringData',
                    text: data.subtype.value.text,
                };
            }
            case 'typedMessage': {
                return {
                    type: 'typedMessage',
                };
            }
            case 'errorResponse': {
                return this.translateErrorResponse(data.subtype.value);
            }
        }
    }

    private translateErrorResponse(e: searchium_pb.ErrorResponse): ipc.ErrorResponse;
    private translateErrorResponse(e: undefined): undefined;
    private translateErrorResponse(e: searchium_pb.ErrorResponse | undefined): ipc.ErrorResponse | undefined {
        if (!e) { return undefined; }
        return {
            type: 'errorResponse',
            message: e.message,
            fullTypeName: e.fullTypeName,
            stackTrace: e.stackTrace,
            innerError: e.innerError ? this.translateErrorResponse(e.innerError) : undefined,
        };
    }
}