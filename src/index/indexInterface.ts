export interface IndexClient {
    registerWorkspaceFolder(path: string): Promise<void>;
    unregisterWorkspaceFolder(path: string): Promise<void>;
}