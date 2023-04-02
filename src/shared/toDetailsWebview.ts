export interface FileByExtensionDetails {
    extension: string;
    count: string;
    size: string;
}
export interface LargeFileDetails {
    path: string;
    size: string;
}
export interface ConfigSection {
    path: string;
    name: string;
    contents: string;
}
export interface ProjectDetails {
    rootPath: string;
    numFiles: string;
    numDirectories: string;
    numSearchableFiles: string;
    searchableFilesMB: number;
    numBinaryFiles: string;
    binaryFilesMB: number;
    searchableByExtension: FileByExtensionDetails[];
    largeFiles: LargeFileDetails[];
    binaryByExtension: FileByExtensionDetails[];
    largeBinaries: LargeFileDetails[];
    ignorePaths: ConfigSection;
    ignoreFiles: ConfigSection;
    includeFiles: ConfigSection;
}

export interface DetailsMessage {
    type: "details",
    projects: ProjectDetails[];
}

export type Message = DetailsMessage;