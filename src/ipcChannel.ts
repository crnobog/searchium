import { Socket } from "net";
import { TypedEmitter } from "tiny-typed-emitter";
import * as ipc from './ipc';
import * as ipcEvents from './ipcEvents';
import * as ipcResponses from './ipcResponses';
import * as ipcRequests from './ipcRequests';
import * as searchium_pb from './gen/searchium';
import * as vscode from "vscode";
import { getLogger } from "./logger";
import { Readable } from "stream";

interface IpcChannelEvents {
    'raw': (raw: searchium_pb.IpcMessage) => void;
    'response': (r: ipcResponses.Response) => void;
    'event': (e: ipcEvents.TypedEvent) => void;
    // 'fatalError' : (e : Error) => void;
}

type RequestSuccess = (r: any) => void;
type RequestFailure = (err: any) => void;
type RequestCompletion = [RequestSuccess, RequestFailure];

class IncomingData {
    private buffers: Buffer[] = [];
    private waiters: (() => void)[] = [];
    private index = 0; // current index into buffers[0] to pull data from 

    constructor(private stream: Readable) {
        stream.on('data', (chunk) => {
            this.buffers.push(chunk);
            let takeWaiters = this.waiters;
            this.waiters = [];
            takeWaiters.forEach((f) => f());
        });
    }

    public async readLengthPrefixedMessage(): Promise<Uint8Array> {
        const numBytes = await this.readLengthPrefix();
        const messageBuffer = await this.readBytes(numBytes);
        return messageBuffer;
    }

    private async waitForData(): Promise<void> {
        return new Promise((resolve, reject) => {
            this.waiters.push(resolve);
        });
    }

    private async readLengthPrefix(): Promise<number> {
        let numBytes: number | undefined = 0;
        let shift = 0;
        let more = true;
        while (more) {
            if (this.buffers.length === 0) {
                await this.waitForData();
                this.index = 0;
            }
            let currentBuffer = this.buffers[0];
            if (this.index >= currentBuffer.byteLength) {
                // move on to next buffer 
                this.index = 0;
                this.buffers.shift();
                continue;
            }
            let byte = currentBuffer.readUint8(this.index++);
            more = (byte & 0x80) !== 0;
            byte = byte & 0x7f;
            numBytes = (numBytes | (byte << shift));
            shift += 7;
        }
        return numBytes;
    }

    private async readBytes(numBytes: number): Promise<Uint8Array> {
        let messageData = new Uint8Array(numBytes);
        let targetIndex = 0;
        while (numBytes > 0) {
            if (this.buffers.length === 0) {
                await this.waitForData();
            }
            let currentBuffer = this.buffers[0];
            const endReadIndex = Math.min(currentBuffer.byteLength, this.index + numBytes);
            const bytesRead = endReadIndex - this.index;
            currentBuffer.copy(messageData, 0, this.index, endReadIndex);
            this.index = endReadIndex;
            numBytes -= bytesRead;
            targetIndex += bytesRead;

            if (this.index === currentBuffer.byteLength) {
                this.buffers.shift();
                this.index = 0;
            }
        }
        return messageData;
    }
}

export class IpcChannel extends TypedEmitter<IpcChannelEvents> implements vscode.Disposable {
    socket: Socket;
    sequenceNum: bigint;
    requestCompletions: Map<bigint, RequestCompletion>;
    data: IncomingData;

    constructor(socket: Socket) {
        super();
        this.socket = socket;
        this.sequenceNum = 1n;
        this.requestCompletions = new Map();
        this.data = new IncomingData(this.socket);
    }

    public dispose() {
        this.socket.end();
        this.socket.destroy();
    }

    public async drainDispatch() {
        while (true) {
            let messageBytes = await this.data.readLengthPrefixedMessage();
            try {
                let rawMessage = searchium_pb.IpcMessage.fromBinary(messageBytes);
                this.emit('raw', rawMessage);
                this.dispatchMessage(rawMessage);
            }
            catch (err: any) {
                getLogger().logError`Error dispatching raw message: ${err}`;
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
            let typedMessage: searchium_pb.TypedMessage = {
                className: "Unknown",
                subtype: {
                    oneofKind: "typedRequest",
                    typedRequest: payload.toProto(),
                }
            };
            let raw: searchium_pb.IpcMessage = {
                protocol: "typed-message",
                requestId: this.nextRequestId(),
                data: {
                    subtype: {
                        oneofKind: "typedMessage", typedMessage,
                    }
                },
                requestResponse: {
                    oneofKind: "request", request: {
                        runOnSequentialQueue: sequential ?? false,
                    }
                }
            };
            const messageBytes = searchium_pb.IpcMessage.toBinary(raw);
            let numBytes = messageBytes.length;
            const messagePrefix = [];
            do {
                let byte = (numBytes & 0x7f);
                numBytes = numBytes >> 7;
                if (numBytes === 0) {
                    messagePrefix.push(byte);
                }
                else {
                    messagePrefix.push(0x80 | byte);
                }
            } while (numBytes !== 0);
            this.requestCompletions.set(raw.requestId, [resolve, reject]);
            this.socket.write(new Uint8Array(messagePrefix), (err) => {
                if (err) { reject(err); }
            });
            this.socket.write(messageBytes, (err) => {
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
        switch (msg.requestResponse.oneofKind) {
            case undefined: throw new Error("Received unknown ipc message type");
            case "request": throw new Error("Received unexpected ipc request");
            case "response":
                if (msg.requestResponse.response.ipcEvent) {
                    return this.dispatchEvent(msg);
                }
                else {
                    return this.dispatchResponse(msg);
                }
        }
    }

    private dispatchEvent(msg: searchium_pb.IpcMessage) {
        // TODO: Full translation for events 
        switch (msg.data?.subtype.oneofKind) {
            case undefined: throw new Error("Missing payload in event");
            case "errorResponse": throw new Error("Unexpected error response payload in event");
            case "ipcStringData": throw new Error("Unexpected string data payload in event");
            case "typedMessage":
                let typedMessage = msg.data.subtype.typedMessage;
                switch (typedMessage.subtype.oneofKind) {
                    case undefined: throw new Error("Missing typed message payload in event");
                    case "typedRequest": throw new Error("Unexpected request paylod in event");
                    case "typedResponse": throw new Error("Unexpected response payload in event");
                    case "typedEvent":
                        let e: ipcEvents.TypedEvent = this.translateTypedEvent(msg.requestId, typedMessage.subtype.typedEvent);
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
        switch (msg.data.subtype.oneofKind) {
            case undefined: throw new Error("Unknown payload for response");
            case "errorResponse": {
                // complete with failure
                let err = this.translateErrorResponse(msg.data.subtype.errorResponse);
                if (failure) {
                    failure(err.message);
                }
                // TODO: Emit global event
                break;
            };
            case "ipcStringData": {
                // complete with string response
                let s = msg.data.subtype.ipcStringData.text;
                if (success) {
                    success(s);
                }
                // TODO: Emit global event 
                break;
            };
            case "typedMessage": {
                // complete with structured message 
                let typedMessage = msg.data.subtype.typedMessage;
                switch (typedMessage.subtype.oneofKind) {
                    case undefined: throw new Error("Undefined typed message type");
                    case 'typedEvent': throw new Error("Unexpected event payloda in response");
                    case 'typedRequest': throw new Error("Unexpected request payload in response");
                    case 'typedResponse':
                        let r = this.translateTypedResponse(requestId, typedMessage.subtype.typedResponse);
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
        switch (data.subtype.oneofKind) {
            case undefined: throw new Error("Undefined event type");
            case 'indexingServerStateChangedEvent':
                return {
                    eventType: "indexingServerStateChanged",
                    requestId,
                    lastIndexUpdatedUtc: new Date(Date.now()), // TODO: translate data.subtype.value.lastIndexUpdatedUtc,
                    serverStatus: data.subtype.indexingServerStateChangedEvent.serverStatus,
                };
            case 'progressReportEvent':
                return { 
                    eventType: "progressReport",
                    requestId,
                    ...data.subtype.progressReportEvent,
                };
            case 'pairedTypedEvent':
                let paired: searchium_pb.PairedTypedEvent = data.subtype.pairedTypedEvent;
                switch (paired.subtype.oneofKind) {
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
                        oldVersion: paired.subtype.fileSystemScanFinished.oldVersion,
                        newVersion: paired.subtype.fileSystemScanFinished.newVersion,
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
                        treeVersion: paired.subtype.searchEngineFilesLoaded.treeVersion,
                        error: paired.error ? this.translateErrorResponse(paired.error) : undefined,
                    };
                }
                break;
        }
        throw new Error("Undefined event type");
    }
    private translateTypedResponse(requestId: bigint, data: searchium_pb.TypedResponse): ipcResponses.Response {
        switch (data.subtype.oneofKind) {
            case undefined: throw new Error("Missing response type in payload");
            case "doneResponse": {
                return {
                    responseType: "done",
                    requestId,
                    info: data.subtype.doneResponse.info
                };
            }
            case "searchCodeResponse": {
                return {
                    responseType: "searchCode",
                    requestId,
                    searchResults: data.subtype.searchCodeResponse.searchResults!, // todo: handle empty
                    hitCount: data.subtype.searchCodeResponse.hitCount,
                    searchedFileCount: data.subtype.searchCodeResponse.searchedFileCount,
                    totalFileCount: data.subtype.searchCodeResponse.totalFileCount,
                };
            }
            case 'getFileExtractsResponse': {
                return {
                    responseType: "getFileExtracts",
                    fileName: data.subtype.getFileExtractsResponse.fileName,
                    fileExtracts: data.subtype.getFileExtractsResponse.fileExtracts ?? [],
                    requestId
                };
            }
            case 'getDatabaseStatisticsResponse': {
                return {
                    requestId,
                    responseType: "getDatabaseStatistics",
                    projectCount: data.subtype.getDatabaseStatisticsResponse.projectCount,
                    fileCount: data.subtype.getDatabaseStatisticsResponse.fileCount,
                    searchableFileCount: data.subtype.getDatabaseStatisticsResponse.searchableFileCount,
                    serverNativeMemoryUsage: data.subtype.getDatabaseStatisticsResponse.serverNativeMemoryUsage,
                    serverGcMemoryUsage: data.subtype.getDatabaseStatisticsResponse.serverGcMemoryUsage,
                    lastIndexUpdatedUtc: new Date(Date.now()), // TODO
                    serverStatus: data.subtype.getDatabaseStatisticsResponse.serverStatus,
                };
            }
            case 'searchFilePathsResponse': {
                return {
                    requestId,
                    responseType: "searchFilePaths",
                    hitCount: data.subtype.searchFilePathsResponse.hitCount,
                    totalCount: data.subtype.searchFilePathsResponse.totalCount,
                    searchResult: data.subtype.searchFilePathsResponse.searchResult!, // todo: handle empty
                };
            }
            case 'getDatabaseDetailsResponse': {
                return {
                    requestId,
                    responseType: "getDatabaseDetails",
                    projects: data.subtype.getDatabaseDetailsResponse.projectDetails,
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