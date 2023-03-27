import { Socket } from "net";
import { TypedEmitter } from "tiny-typed-emitter";
import { PlainMessage, protoDelimited } from "@bufbuild/protobuf";
import * as ipc from './ipc';
import * as ipcEvents from './ipcEvents';
import * as ipcResponses from './ipcResponses';
import * as ipcRequests from './ipcRequests';
import * as searchium_pb from './gen/searchium_pb';
import * as vscode from "vscode";
import { getLogger } from "./logger";

interface IpcChannelEvents {
    'raw': (raw: searchium_pb.IpcMessage) => void;
    'response': (r: ipcResponses.Response) => void;
    'event': (e: ipcEvents.TypedEvent) => void;
    // 'fatalError' : (e : Error) => void;
}

type RequestSuccess = (r: any) => void;
type RequestFailure = (err: any) => void;
type RequestCompletion = [RequestSuccess, RequestFailure];

export class IpcChannel extends TypedEmitter<IpcChannelEvents> implements vscode.Disposable {
    socket: Socket;
    sequenceNum: bigint;
    requestCompletions: Map<bigint, RequestCompletion>;

    constructor(socket: Socket) {
        super();
        this.socket = socket;
        this.sequenceNum = 1n;
        this.requestCompletions = new Map();
    }

    public dispose() {
        this.socket.end();
        this.socket.destroy();
    }

    public async drainDispatch() {
        for await (const rawMessage of protoDelimited.decStream(searchium_pb.IpcMessage, this.socket)) {
            try {
                this.emit('raw', rawMessage);
                this.dispatchMessage(rawMessage);
            }
            catch (err: any) {
                getLogger().log`Error dispatching raw message ${rawMessage}: ${err}`;
            }
        }
    }

    // TODO: map request types to response types 
    public async sendSequentialRequest(payload: ipcRequests.Request): Promise<void> {
        return await this.sendRequest(payload, true);
    }

    // TODO: typing for responses 
    public async sendRequest(payload: ipcRequests.Request, sequential?: boolean): Promise<any> {
        return new Promise((resolve, reject) => {
            let typedMessage: PlainMessage<searchium_pb.TypedMessage> = {
                className: "Unknown",
                subtype: {
                    case: "typedRequest",
                    value: payload.toProto(),
                }
            };
            let raw = new searchium_pb.IpcMessage({
                protocol: "typed-message",
                requestId: this.nextRequestId(),
                data: {
                    subtype: {
                        case: "typedMessage", value: typedMessage,
                    }
                },
                requestResponse: {
                    case: "request", value: {
                        runOnSequentialQueue: sequential,
                    }
                }
            });
            const bytes = protoDelimited.enc(raw);
            this.requestCompletions.set(raw.requestId, [resolve, reject]);
            this.socket.write(bytes, (err) => {
                if (err) { reject(err); }
            });
        });
    }

    private nextRequestId(): bigint {
        this.sequenceNum += 1n;
        return this.sequenceNum;
    }

    private dispatchMessage(msg: searchium_pb.IpcMessage) {
        let response: searchium_pb.IpcResponse;
        switch (msg.requestResponse.case) {
            case undefined: throw new Error("Received unknown ipc message type");
            case "request": throw new Error("Received unexpected ipc request");
            case "response":
                if (msg.requestResponse.value.ipcEvent) {
                    return this.dispatchEvent(msg);
                }
                else {
                    return this.dispatchResponse(msg);
                }
        }
    }

    private dispatchEvent(msg: searchium_pb.IpcMessage) {
        // TODO: Full translation for events 
        switch (msg.data?.subtype.case) {
            case undefined: throw new Error("Missing payload in event");
            case "errorResponse": throw new Error("Unexpected error response payload in event");
            case "ipcStringData": throw new Error("Unexpected string data payload in event");
            case "typedMessage":
                let typedMessage = msg.data.subtype.value;
                switch (typedMessage.subtype.case) {
                    case undefined: throw new Error("Missing typed message payload in event");
                    case "typedRequest": throw new Error("Unexpected request paylod in event");
                    case "typedResponse": throw new Error("Unexpected response payload in event");
                    case "typedEvent":
                        let e: ipcEvents.TypedEvent = this.translateTypedEvent(msg.requestId, typedMessage.subtype.value);
                        this.emit('event', e);
                }
        }
    }

    private dispatchResponse(msg: searchium_pb.IpcMessage) {
        if (!msg.data) {
            throw new Error("Missing payload for response");
        }
        let requestId = msg.requestId;
        let completion = this.requestCompletions.get(requestId);
        if (completion) {
            this.requestCompletions.delete(requestId);
        }
        let [success, failure] = completion ?? [undefined, undefined];
        switch (msg.data.subtype.case) {
            case undefined: throw new Error("Unknown payload for response");
            case "errorResponse": {
                // complete with failure
                let err = this.translateErrorResponse(msg.data.subtype.value);
                if (failure) {
                    failure(err.message);
                }
                // TODO: Emit global event
                break;
            };
            case "ipcStringData": {
                // complete with string response
                let s = msg.data.subtype.value.text;
                if (success) {
                    success(s);
                }
                // TODO: Emit global event 
                break;
            };
            case "typedMessage": {
                // complete with structured message 
                let typedMessage = msg.data.subtype.value;
                switch (typedMessage.subtype.case) {
                    case undefined: throw new Error("Undefined typed message type");
                    case 'typedEvent': throw new Error("Unexpected event payloda in response");
                    case 'typedRequest': throw new Error("Unexpected request payload in response");
                    case 'typedResponse':
                        let r = this.translateTypedResponse(requestId, typedMessage.subtype.value);
                        this.emit('response', r);
                        if (success) {
                            success(r);
                        }
                        break;
                }
                // TODO: Emit global event 
                break;
            }
        }
    }

    private translateTypedEvent(requestId: bigint, data: searchium_pb.TypedEvent): ipcEvents.TypedEvent {
        switch (data.subtype.case) {
            case undefined: throw new Error("Undefined event type");
            case 'indexingServerStateChangedEvent':
                return {
                    eventType: "indexingServerStateChanged",
                    requestId,
                    lastIndexUpdatedUtc: new Date(Date.now()), // TODO: translate data.subtype.value.lastIndexUpdatedUtc,
                    serverStatus: data.subtype.value.serverStatus,
                };
            case 'progressReportEvent':
                return { 
                    eventType: "progressReport",
                    requestId,
                    ...data.subtype.value,
                };
            case 'pairedTypedEvent':
                let paired: searchium_pb.PairedTypedEvent = data.subtype.value;
                switch (paired.subtype.case) {
                    case 'fileSystemScanStarted': return {
                        eventType: "fileSystemScanStarted",
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                        operationId: paired.operationId,
                        requestId,
                    };
                    case 'fileSystemScanFinished': return {
                        eventType: "fileSystemScanFinished",
                        requestId,
                        operationId: paired.operationId,
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                        oldVersion: paired.subtype.value.oldVersion,
                        newVersion: paired.subtype.value.newVersion,
                    };
                    case 'searchEngineFilesLoading': return {
                        eventType: "searchEngineFilesLoading",
                        operationId: paired.operationId,
                        requestId,
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                    };
                    case 'searchEngineFilesLoadingProgress': return {
                        eventType: "searchEngineFilesLoadingProgress",
                        operationId: paired.operationId,
                        requestId,
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                    };
                    case 'searchEngineFilesLoaded': return {
                        eventType: "searchEngineFilesLoaded",
                        operationId: paired.operationId,
                        requestId,
                        treeVersion: paired.subtype.value.treeVersion,
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                    };
                }
                break;
        }
        throw new Error("Undefined event type");
    }
    private translateTypedResponse(requestId: bigint, data: searchium_pb.TypedResponse): ipcResponses.Response {
        switch (data.subtype.case) {
            case undefined: throw new Error("Missing response type in payload");
            case "doneResponse": {
                return {
                    responseType: "done",
                    requestId,
                    info: data.subtype.value.info
                };
            }
            case "searchCodeResponse": {
                return {
                    responseType: "searchCode",
                    requestId,
                    searchResults: data.subtype.value.searchResults!, // todo: handle empty
                    hitCount: data.subtype.value.hitCount,
                    searchedFileCount: data.subtype.value.searchedFileCount,
                    totalFileCount: data.subtype.value.totalFileCount,
                };
            }
            case 'getFileExtractsResponse': {
                return {
                    responseType: "getFileExtracts",
                    fileName: data.subtype.value.fileName,
                    fileExtracts: data.subtype.value.fileExtracts ?? [],
                    requestId
                };
            }
            case 'getDatabaseStatisticsResponse': {
                return {
                    requestId,
                    responseType: "getDatabaseStatistics",
                    projectCount: data.subtype.value.projectCount,
                    fileCount: data.subtype.value.fileCount,
                    searchableFileCount: data.subtype.value.searchableFileCount,
                    serverNativeMemoryUsage: data.subtype.value.serverNativeMemoryUsage,
                    serverGcMemoryUsage: data.subtype.value.serverGcMemoryUsage,
                    lastIndexUpdatedUtc: new Date(Date.now()), // TODO
                    serverStatus: data.subtype.value.serverStatus,
                };
            }
        }
        throw new Error("TODO");
    }

    private translateErrorResponse(e: searchium_pb.ErrorResponse): ipc.ErrorResponse;
    private translateErrorResponse(e: undefined): undefined;
    private translateErrorResponse(e: searchium_pb.ErrorResponse | undefined): ipc.ErrorResponse | undefined {
        if (!e) { return undefined; }
        return {
            dataType: 'errorResponse',
            message: e.message,
            fullTypeName: e.fullTypeName,
            stackTrace: e.stackTrace,
            innerError: e.innerError ? this.translateErrorResponse(e.innerError) : undefined,
        };
    }
}