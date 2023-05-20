export interface DetailsMessage {
    type: "details",
    roots: DatabaseDetailsRoot[],
}
export interface DatabaseDetailsRoot {
    rootPath: string,
    numFilesScanned: string,
    numDirectoriesScanned: string,
    numSearchableFiles: string,
    searchableFilesMB: number,
    numBinaryFiles: string,
    binaryFilesMB: number,
    searchableFilesByExtension: FilesByExtensionDetails[],
    binaryFilesByExtension: FilesByExtensionDetails[],
    largeFiles: LargeFileDetails[],
    largeBinaries: LargeFileDetails[],
}
export interface FilesByExtensionDetails {
    extension: string,
    count: string,
    mb: number,
}
export interface LargeFileDetails {
    path: string,
    sizeMb: number,
}
export type Message = DetailsMessage;