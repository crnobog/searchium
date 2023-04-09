import { IpcChannel } from "./ipcChannel";
import * as vscode from "vscode";
import { SearchFilePathsRequest } from "./ipcRequests";
import { SearchFilePathsResponse } from "./ipcResponses";

class FileResult implements vscode.QuickPickItem {
    kind?: vscode.QuickPickItemKind | undefined;
    description?: string | undefined;
    detail?: string | undefined;
    picked?: boolean | undefined;
    alwaysShow?: boolean | undefined;
    buttons?: readonly vscode.QuickInputButton[] | undefined;

    constructor(public label: string) {

    }
}

export class FileSearchManager {
    constructor(private channel: IpcChannel) { }

    public async onSearchFilePaths(): Promise<void> {
        const disposables: vscode.Disposable[] = [];
        try {
            await new Promise<vscode.Uri | undefined>((_resolve, _reject) => {
                const input = vscode.window.createQuickPick<FileResult>();
                input.canSelectMany = false;
                input.placeholder = "Type to search for files";
                // TODO set busy when searching 

                let cancelSource = new vscode.CancellationTokenSource();
                disposables.push(cancelSource);
                let timeout: NodeJS.Timeout | undefined = undefined;
                const debounceTime = 200;
                input.onDidChangeValue((value) => {
                    // todo: debounce searching  
                    if (!value || value === "") {
                        input.items = [];
                        return;
                    }
                    if (timeout) { clearTimeout(timeout); }
                    cancelSource.cancel();
                    cancelSource = new vscode.CancellationTokenSource();
                    timeout = setTimeout(async () => {
                        input.busy = true;
                        const results = await this.searchForFiles(value, cancelSource.token);
                        if (!results) { return; } // cancelled
                        input.busy = false;
                        input.items = results;
                        // todo: cancellation? 
                    }, debounceTime);
                }, disposables);

                input.show();
            });
        }
        finally {
            disposables.forEach(d => d.dispose());
        }
    }

    private async searchForFiles(value: string, token: vscode.CancellationToken): Promise<FileResult[] | undefined> {
        const results = await this.channel.sendRequest(new SearchFilePathsRequest({
            searchString: value,
            includeSymLinks: false,
            matchCase: false,
            matchWholeWord: false,
            maxResults: 100,
            regex: true,
            useRe2Engine: false,
            filePathPattern: "",
        })) as SearchFilePathsResponse;
        if (token.isCancellationRequested) { return undefined; }

        if (results.hitCount === 0n) {
            return [];
        }

        if (results.searchResult.subtype.oneofKind !== 'directoryEntry') {
            return []; // TODO error 
        }
        const root = results.searchResult.subtype.directoryEntry;
        const files = [];
        for (const dir of root.entries) {
            if (dir.subtype.oneofKind !== 'directoryEntry') { continue; }
            // TODO: dividor/group per directory/project 
            for (const file of dir.subtype.directoryEntry.entries) {
                if (file.subtype.oneofKind !== 'fileEntry') { continue; }
                files.push(new FileResult(file.name));
            }
        }
        return files;
    }
}