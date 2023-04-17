import * as vscode from "vscode";
import { IndexClient } from "index/indexInterface";
import { getLogger } from "logger";
import * as pb from 'gen/searchium/v2/searchium';

class FileResult implements vscode.QuickPickItem {
    kind?: vscode.QuickPickItemKind | undefined;
    description?: string | undefined;
    detail?: string | undefined;
    picked?: boolean | undefined;
    alwaysShow?: boolean | undefined;
    buttons?: readonly vscode.QuickInputButton[] | undefined;

    constructor(public label: string) {
        this.alwaysShow = true;
    }
}

class Debouncer implements vscode.Disposable {
    handle?: NodeJS.Timeout;

    constructor(private debounceTimeMS: number) {
    }

    public dispose(): void {
        if (this.handle) {
            clearTimeout(this.handle);
        }
    }

    public do(func: () => Promise<void>): void {
        if (this.handle) {
            clearTimeout(this.handle);
        }
        this.handle = setTimeout(func, this.debounceTimeMS);
    }
}

class ResultsWaiter implements vscode.Disposable {
    cancelSource: vscode.CancellationTokenSource;
    resolve?: (r: pb.FilePathSearchResponse) => void;

    constructor(private results: AsyncIterable<pb.FilePathSearchResponse>) {
        this.cancelSource = new vscode.CancellationTokenSource();
        process.nextTick(async () => {
            for await (const result of this.results) {
                if (this.cancelSource.token.isCancellationRequested) {
                    getLogger().logInformation`Canceling file search results processing`;
                    break;
                }
                getLogger().logInformation`Got new file search results in ${result.duration?.seconds.toString()}s`;
                for (const path of result.results) {
                    getLogger().logInformation`${path}`;
                }
                if (this.resolve) {
                    this.resolve(result);
                }
            }
        });
    }

    public dispose(): void {
        this.cancelSource.cancel();
    }

    public waitForNext(): Promise<pb.FilePathSearchResponse> {
        return new Promise((resolve, _reject) => this.resolve = resolve);
    }
}

export class FileSearchManager implements vscode.Disposable {
    constructor(private client: IndexClient) { }

    public dispose(): void { getLogger().logDebug`Disposing FileSearchManager`; }

    public async onSearchFilePaths(): Promise<void> {
        const disposables: vscode.Disposable[] = [];
        try {
            await new Promise<vscode.Uri | undefined>((_resolve, _reject) => {
                const input = vscode.window.createQuickPick<FileResult>();
                input.canSelectMany = false;
                input.placeholder = "Type to search for files";
                // TODO set busy when searching 
                const stream = this.client.searchFilePaths();
                const debouncer = new Debouncer(200);
                const waiter = new ResultsWaiter(stream.results);
                disposables.push(
                    input,
                    debouncer,
                    waiter,
                    {
                        dispose: () => { stream.complete(); },
                    });
                input.onDidChangeValue((value) => {
                    // todo: debounce searching  
                    if (!value || value === "") {
                        return;
                    }
                    // TODO: set input busy 
                    debouncer.do(async () => {
                        await stream.send({ query: value, maxResults: 20 });
                        // todo: cancellation? 
                        const r = await waiter.waitForNext();
                        input.items = r.results.map((path) => new FileResult(path));
                    });
                }, disposables);


                input.show();
            });
        }
        finally {
            disposables.forEach(d => d.dispose());
        }
    }
}