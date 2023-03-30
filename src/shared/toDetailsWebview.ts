
export interface ProjectDetails {
    rootPath: string;
    numFiles: string;
    numDirectories: string;
    numSearchableFiles: string;
    searchableFilesMB: number;
    numBinaryFiles: string;
    binaryFilesMB: number;
}

export interface DetailsMessage {
    type: "details",
    projects: ProjectDetails[];
}

export type Message = DetailsMessage;