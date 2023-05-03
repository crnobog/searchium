/* eslint-disable */
// @generated by protobuf-ts 2.9.0 with parameter output_legacy_commonjs,eslint_disable
// @generated from protobuf file "searchium/v2/searchium.proto" (package "searchium.v2", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { SearchiumService } from "./searchium";
import type { ProcessInfoResponse } from "./searchium";
import type { ProcessInfoRequest } from "./searchium";
import type { ConfigurationResponse } from "./searchium";
import type { ConfigurationRequest } from "./searchium";
import type { FileExtractsResponse } from "./searchium";
import type { FileExtractsRequest } from "./searchium";
import type { FileContentsSearchResponse } from "./searchium";
import type { FileContentsSearchRequest } from "./searchium";
import type { FilePathSearchResponse } from "./searchium";
import type { FilePathSearchRequest } from "./searchium";
import type { DuplexStreamingCall } from "@protobuf-ts/runtime-rpc";
import type { GenericResponse } from "./searchium";
import type { FolderUnregisterRequest } from "./searchium";
import type { IndexUpdate } from "./searchium";
import type { FolderRegisterRequest } from "./searchium";
import type { ServerStreamingCall } from "@protobuf-ts/runtime-rpc";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { HelloResponse } from "./searchium";
import type { HelloRequest } from "./searchium";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service searchium.v2.SearchiumService
 */
export interface ISearchiumServiceClient {
    /**
     * TODO: Replace with capabilities/version negotiation
     *
     * @generated from protobuf rpc: Hello(searchium.v2.HelloRequest) returns (searchium.v2.HelloResponse);
     */
    hello(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse>;
    /**
     * @generated from protobuf rpc: RegisterFolder(searchium.v2.FolderRegisterRequest) returns (stream searchium.v2.IndexUpdate);
     */
    registerFolder(input: FolderRegisterRequest, options?: RpcOptions): ServerStreamingCall<FolderRegisterRequest, IndexUpdate>;
    /**
     * @generated from protobuf rpc: UnregisterFolder(searchium.v2.FolderUnregisterRequest) returns (searchium.v2.GenericResponse);
     */
    unregisterFolder(input: FolderUnregisterRequest, options?: RpcOptions): UnaryCall<FolderUnregisterRequest, GenericResponse>;
    /**
     * @generated from protobuf rpc: SearchFilePaths(stream searchium.v2.FilePathSearchRequest) returns (stream searchium.v2.FilePathSearchResponse);
     */
    searchFilePaths(options?: RpcOptions): DuplexStreamingCall<FilePathSearchRequest, FilePathSearchResponse>;
    /**
     * @generated from protobuf rpc: SearchFileContents(searchium.v2.FileContentsSearchRequest) returns (stream searchium.v2.FileContentsSearchResponse);
     */
    searchFileContents(input: FileContentsSearchRequest, options?: RpcOptions): ServerStreamingCall<FileContentsSearchRequest, FileContentsSearchResponse>;
    /**
     * @generated from protobuf rpc: GetFileExtracts(searchium.v2.FileExtractsRequest) returns (searchium.v2.FileExtractsResponse);
     */
    getFileExtracts(input: FileExtractsRequest, options?: RpcOptions): UnaryCall<FileExtractsRequest, FileExtractsResponse>;
    /**
     * @generated from protobuf rpc: SetConfiguration(searchium.v2.ConfigurationRequest) returns (searchium.v2.ConfigurationResponse);
     */
    setConfiguration(input: ConfigurationRequest, options?: RpcOptions): UnaryCall<ConfigurationRequest, ConfigurationResponse>;
    /**
     * @generated from protobuf rpc: GetProcessInfo(searchium.v2.ProcessInfoRequest) returns (searchium.v2.ProcessInfoResponse);
     */
    getProcessInfo(input: ProcessInfoRequest, options?: RpcOptions): UnaryCall<ProcessInfoRequest, ProcessInfoResponse>;
}
/**
 * @generated from protobuf service searchium.v2.SearchiumService
 */
export class SearchiumServiceClient implements ISearchiumServiceClient, ServiceInfo {
    typeName = SearchiumService.typeName;
    methods = SearchiumService.methods;
    options = SearchiumService.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * TODO: Replace with capabilities/version negotiation
     *
     * @generated from protobuf rpc: Hello(searchium.v2.HelloRequest) returns (searchium.v2.HelloResponse);
     */
    hello(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<HelloRequest, HelloResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: RegisterFolder(searchium.v2.FolderRegisterRequest) returns (stream searchium.v2.IndexUpdate);
     */
    registerFolder(input: FolderRegisterRequest, options?: RpcOptions): ServerStreamingCall<FolderRegisterRequest, IndexUpdate> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<FolderRegisterRequest, IndexUpdate>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: UnregisterFolder(searchium.v2.FolderUnregisterRequest) returns (searchium.v2.GenericResponse);
     */
    unregisterFolder(input: FolderUnregisterRequest, options?: RpcOptions): UnaryCall<FolderUnregisterRequest, GenericResponse> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<FolderUnregisterRequest, GenericResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: SearchFilePaths(stream searchium.v2.FilePathSearchRequest) returns (stream searchium.v2.FilePathSearchResponse);
     */
    searchFilePaths(options?: RpcOptions): DuplexStreamingCall<FilePathSearchRequest, FilePathSearchResponse> {
        const method = this.methods[3], opt = this._transport.mergeOptions(options);
        return stackIntercept<FilePathSearchRequest, FilePathSearchResponse>("duplex", this._transport, method, opt);
    }
    /**
     * @generated from protobuf rpc: SearchFileContents(searchium.v2.FileContentsSearchRequest) returns (stream searchium.v2.FileContentsSearchResponse);
     */
    searchFileContents(input: FileContentsSearchRequest, options?: RpcOptions): ServerStreamingCall<FileContentsSearchRequest, FileContentsSearchResponse> {
        const method = this.methods[4], opt = this._transport.mergeOptions(options);
        return stackIntercept<FileContentsSearchRequest, FileContentsSearchResponse>("serverStreaming", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: GetFileExtracts(searchium.v2.FileExtractsRequest) returns (searchium.v2.FileExtractsResponse);
     */
    getFileExtracts(input: FileExtractsRequest, options?: RpcOptions): UnaryCall<FileExtractsRequest, FileExtractsResponse> {
        const method = this.methods[5], opt = this._transport.mergeOptions(options);
        return stackIntercept<FileExtractsRequest, FileExtractsResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: SetConfiguration(searchium.v2.ConfigurationRequest) returns (searchium.v2.ConfigurationResponse);
     */
    setConfiguration(input: ConfigurationRequest, options?: RpcOptions): UnaryCall<ConfigurationRequest, ConfigurationResponse> {
        const method = this.methods[6], opt = this._transport.mergeOptions(options);
        return stackIntercept<ConfigurationRequest, ConfigurationResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: GetProcessInfo(searchium.v2.ProcessInfoRequest) returns (searchium.v2.ProcessInfoResponse);
     */
    getProcessInfo(input: ProcessInfoRequest, options?: RpcOptions): UnaryCall<ProcessInfoRequest, ProcessInfoResponse> {
        const method = this.methods[7], opt = this._transport.mergeOptions(options);
        return stackIntercept<ProcessInfoRequest, ProcessInfoResponse>("unary", this._transport, method, opt, input);
    }
}
