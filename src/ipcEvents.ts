import { ErrorResponse } from "./ipc";
import * as searchium_pb from './gen/searchium';

export interface Event {
    requestId: bigint;
}

export interface PairedTypedEvent extends Event {
    operationId: bigint;
    error?: ErrorResponse
}

export interface FileSystemScanStartedEvent extends PairedTypedEvent {
    eventType: 'fileSystemScanStarted';
}

export interface FileSystemScanFinishedEvent extends PairedTypedEvent {
    eventType: 'fileSystemScanFinished';
    oldVersion: number;
    newVersion: number;
}

export interface SearchEngineFilesLoadingEvent extends PairedTypedEvent {
    eventType: 'searchEngineFilesLoading';
}

export interface SearchEngineFilesLoadingProgressEvent extends PairedTypedEvent {
    eventType: 'searchEngineFilesLoadingProgress';
}

export interface SearchEngineFilesLoadedEvent extends PairedTypedEvent {
    eventType: 'searchEngineFilesLoaded';
    treeVersion: bigint;
}

export interface ProgressReportEvent extends Event {
    eventType: 'progressReport';
    displayText: string;
    completed: number;
    total: number;
}

export interface IndexingServerStateChangedEvent extends Event {
    eventType: 'indexingServerStateChanged';
    serverStatus: searchium_pb.IndexingServerStatus;
    lastIndexUpdatedUtc: Date;
}

export type TypedEvent = FileSystemScanStartedEvent
    | FileSystemScanFinishedEvent
    | SearchEngineFilesLoadedEvent
    | SearchEngineFilesLoadingEvent
    | SearchEngineFilesLoadingProgressEvent
    | ProgressReportEvent
    | IndexingServerStateChangedEvent;