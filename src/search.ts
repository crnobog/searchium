import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import * as ipcRequests from "./ipcRequests";
import * as ipcResponses from "./ipcResponses";
import { getLogger } from "./logger";
import * as searchium_pb from "./gen/searchium";
import * as path from "path";
import assert = require("assert");
import './extensionsMethods';
import { SearchHistory } from "./history";

export interface SearchOptions {
    query: string,
    pathFilter: string,
    matchCase: boolean,
    wholeWord: boolean,
    regex: boolean,
}

interface DirectoryResult {
    type: 'directory';
    name: string;
    path: string;
    children: FileResult[];
    parent: undefined;
    next?: DirectoryResult;
    prev?: DirectoryResult;
}
interface FileResult {
    type: 'file';
    name: string;
    path: string;
    uri: vscode.Uri;
    positions: searchium_pb.FilePositionSpan[];
    extracts: () => Promise<ExtractResult[]>;
    parent: DirectoryResult,
    next?: FileResult;
    prev?: FileResult;
}
type ExtractResult = {
    type: 'extract';
    highlights: [number, number][];
    range: vscode.Range;
    parent: FileResult;
    next?: ExtractResult;
    prev?: ExtractResult;
    text: string;
    lineNumber: number;
    columnNumber: number;
};

type SearchResult = DirectoryResult | FileResult | ExtractResult;

async function getFileExtracts(channel: IpcChannel, file: FileResult) {
    let extracts = (await channel.sendRequest(new ipcRequests.GetFileExtractsRequest(file.path, file.positions, 100))
        .then((r: ipcResponses.GetFileExtractsResponse) =>
            r.fileExtracts))
        .map((e, i) =>
            convertFileExtracts(file, e, file.positions[i]));
    for (let i = 0; i < extracts.length; ++i) {
        if (i !== 0) {
            extracts[i].prev = extracts[i - 1];
        }
        if (i !== extracts.length - 1) {
            extracts[i].next = extracts[i + 1];
        }
    }
    return extracts;
}

function convertFileResult(
    channel: IpcChannel,
    parent: DirectoryResult,
    entry: searchium_pb.FileSystemEntry,
    parentPath?: string): FileResult {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.oneofKind) {
        case 'directoryEntry':
            throw new Error("Unexpected directory entry");
        case 'fileEntry':
            let file: FileResult & { _extracts: Promise<ExtractResult[]> | undefined } = {
                type: 'file',
                name: entry.name,
                path: thisPath,
                parent: parent,
                uri: vscode.Uri.file(thisPath),
                positions: entry.data?.filePositionsData?.positions ?? [],
                _extracts: undefined,
                extracts: async function (): Promise<ExtractResult[]> {
                    if (this._extracts) { this._extracts; }
                    this._extracts = getFileExtracts(channel, this);
                    return this._extracts;
                }
            };
            return file;
        case undefined: throw new Error("Unknown filesystement type");
    }
}

function convertDirectoryResult(
    channel: IpcChannel,
    entry: searchium_pb.FileSystemEntry,
    parentPath?: string
): DirectoryResult {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.oneofKind) {
        case 'directoryEntry':
            let dir: DirectoryResult = {
                type: 'directory',
                name: entry.name,
                path: thisPath,
                parent: undefined,
                children: []
            };
            dir.children = entry.subtype.directoryEntry.entries.map((e) =>
                convertFileResult(channel, dir, e, thisPath)
            );
            for (let i = 0; i < dir.children.length; ++i) {
                if (i !== 0) {
                    dir.children[i].prev = dir.children[i - 1];
                }
                if (i !== dir.children.length - 1) {
                    dir.children[i].next = dir.children[i + 1];
                }
            }
            return dir;
        case 'fileEntry':
            throw new Error("Unxpected file entry");
        case undefined: throw new Error("Unknown filesystement type");
    }
}

function convertFileExtracts(parent: FileResult, extract: searchium_pb.FileExtract, info: searchium_pb.FilePositionSpan): ExtractResult {
    let text = extract.text.trimStart();
    let trimmed = extract.text.length - text.length;
    let start = info.position - extract.offset;
    let end = start + info.length;
    let range =
        new vscode.Range(extract.lineNumber, extract.columnNumber, extract.lineNumber, extract.columnNumber + end - start);
    return {
        type: "extract",
        highlights: [[start - trimmed, end - trimmed]],
        parent,
        range,
        text: text.trimEnd(),
        columnNumber: extract.columnNumber,
        lineNumber: extract.lineNumber
    };
}

// TODO: Show progress bar on treeview or elsewhere if indexing is still in progress
export class SearchResultsProvider implements vscode.TreeDataProvider<SearchResult> {
    private _onDidChangeTreeData: vscode.EventEmitter<SearchResult | undefined | void> = new vscode.EventEmitter<SearchResult | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<SearchResult | undefined | void> = this._onDidChangeTreeData.event;

    currentRequestId: bigint = 0n;
    rootResults: SearchResult[] = [];
    treeView?: vscode.TreeView<SearchResult>;

    constructor(private channel: IpcChannel) {
    }

    // TODO: Remove first layer of tree if there's only one project/directory ?
    public populate(r: ipcResponses.SearchCodeResponse, treeView: vscode.TreeView<SearchResult>) {
        if (r.requestId < this.currentRequestId) {
            return;
        }
        this.treeView = treeView;
        this.currentRequestId = r.requestId;
        this.rootResults = [];
        if (r.searchResults.subtype.oneofKind === 'directoryEntry') {
            this.rootResults = r.searchResults.subtype.directoryEntry.entries.map((e) =>
                convertDirectoryResult(this.channel, e)
            );
        }
        this._onDidChangeTreeData.fire(undefined);
    }
    public getTreeItem(element: SearchResult): vscode.TreeItem | Thenable<vscode.TreeItem> {
        switch (element.type) {
            case 'directory': {
                const item = new vscode.TreeItem(element.name, vscode.TreeItemCollapsibleState.Expanded);
                item.resourceUri = vscode.Uri.file(element.path);
                item.iconPath = vscode.ThemeIcon.Folder;
                return item;
            }
            case 'file': {
                const label: vscode.TreeItemLabel = {
                    label: element.name
                };
                const item = new vscode.TreeItem(label, vscode.TreeItemCollapsibleState.Collapsed);
                item.resourceUri = element.uri;
                item.iconPath = vscode.ThemeIcon.File;
                item.description = `${element.positions.length} results`;
                item.id = `${element.path}${this.currentRequestId}`;
                return item;
            }
            case 'extract': {
                const label: vscode.TreeItemLabel = {
                    label: element.text,
                    highlights: element.highlights,
                };
                let item = new vscode.TreeItem(label);
                item.description = `line ${element.lineNumber}`;
                let showOptions: vscode.TextDocumentShowOptions = { preview: false, preserveFocus: false, selection: element.range };
                item.command = {
                    command: "vscode.open",
                    arguments: [element.parent.uri, showOptions],
                    title: `Open ${item.resourceUri}`
                };
                return item;
            }
        }
    }
    public async getChildren(element?: SearchResult): Promise<SearchResult[] | undefined> {
        if (!element) {
            // return search root items
            // this.treeView?.reveal(this.rootResults[0], { select: true, focus: true, expand: false });
            return this.rootResults;
        }

        switch (element.type) {
            case 'directory': return element.children;
            case 'extract': return undefined;
            case 'file': {

                let extracts = (await this.channel.sendRequest(new ipcRequests.GetFileExtractsRequest(element.path, element.positions, 100))
                    .then((r: ipcResponses.GetFileExtractsResponse) => r.fileExtracts))
                    .map((e, i) => convertFileExtracts(element, e, element.positions[i]));
                for (let i = 0; i < extracts.length; ++i) {
                    if (i !== 0) {
                        extracts[i].prev = extracts[i - 1];
                    }
                    if (i !== extracts.length - 1) {
                        extracts[i].next = extracts[i + 1];
                    }
                }
                return extracts;
            }
        }
    }
    public getParent(element: SearchResult): SearchResult | undefined {
        return element.parent;
    }
    // public resolveTreeItem?(item: vscode.TreeItem, element: SearchResult, token: vscode.CancellationToken): vscode.ProviderResult<vscode.TreeItem> {
    //     throw new Error('Method not implemented.');
    // }
}

export class SearchManager {
    private currentNavOperation: Promise<void>;
    constructor(
        private readonly provider: SearchResultsProvider,
        private readonly treeView: vscode.TreeView<SearchResult>,
        private readonly channel: IpcChannel,
        private readonly history: SearchHistory
    ) {
        treeView.onDidChangeSelection(this.onTreeViewSelectionChanged, this);
        this.currentNavOperation = Promise.resolve();
    }

    public async onQuery(options: SearchOptions | undefined, type: "history" | undefined) {
        if (options) {
            return await this.executeSearch(options, type);
        }
        else {
            getLogger().logError`Undefined query string`;
        }
    }

    public async executeSearch(options: Partial<SearchOptions>, type: "history" | undefined): Promise<void> {
        // TODO: cancel previous tasks/deal with spamming search requests
        const maxResults = vscode.workspace.getConfiguration("searchium").get<number>("maxResults", 10000);
        this.treeView.badge = undefined;
        if (type !== "history" && options.query) {
            this.history.add(options.query);
        }
        vscode.commands.executeCommand('searchium-results.focus'); // Reveal first to show progress spinner
        await vscode.window.withProgress({ location: { viewId: 'searchium-results' }, cancellable: false, title: "Searching..." },
            async (progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken): Promise<void> => {
                return await this.channel.sendRequest(new ipcRequests.SearchCodeRequest({
                    searchString: options.query ?? "",
                    filePathPattern: options.pathFilter ?? "",
                    maxResults,
                    matchCase: options.matchCase ?? false,
                    matchWholeWord: options.wholeWord ?? false,
                    includeSymLinks: false,
                    regex: options.regex ?? false,
                    useRe2Engine: false
                }))
                    .then((r: ipcResponses.SearchCodeResponse) => {
                        getLogger().logDebug`Search request complete`;
                        let prefix = "";
                        if (r.hitCount >= maxResults) {
                            vscode.window.showInformationMessage("Search results exceeded configured maximum. Search results will be truncated.");
                            prefix = "More than ";
                        }
                        this.treeView.badge = { tooltip: `${prefix}${r.hitCount.toLocaleString()} search results`, value: Number(r.hitCount) };
                        this.provider.populate(r, this.treeView);
                        if (type !== "history") {
                            this.treeView.reveal(this.provider.rootResults[0], { select: true, focus: false, expand: false });
                        }
                    })
                    .catch((err) => getLogger().logError`Search request failed: ${err}`);
            });
    }

    public async navigateToNextResult() {
        this.addNavigationOperation(() => this.navigateToNextResultInternal());
    }

    private async navigateToNextResultInternal() {
        if (this.treeView.selection.length === 0) {
            if (this.provider.rootResults.length > 0) {
                this.treeView.reveal(this.provider.rootResults[0]);
            }
        }
        else {
            let current = this.treeView.selection[0];
            switch (current.type) {
                case 'directory':
                    {
                        // select first child 
                        this.revealAndPreviewResult((await current.children[0].extracts())[0]);
                    }
                    break;
                case 'file':
                    {
                        this.revealAndPreviewResult((await current.extracts())[0]);
                    }
                    break;
                case 'extract':
                    {
                        // Select next sibling or sibling of parent 
                        if (current.next) {
                            this.revealAndPreviewResult(current.next);
                        }
                        else if (current.parent.next) {
                            this.revealAndPreviewResult((await current.parent.next.extracts())[0]);
                        }
                        else if (current.parent.parent.next) {
                            // select next directory's first result
                            this.revealAndPreviewResult((await current.parent.parent.next.children[0].extracts())[0]);
                        }
                        else {
                            // loop? 
                        }
                    }
                    break;
            }
        }
    }
    public async navigateToPreviousResult() {
        this.addNavigationOperation(() => this.navigateToPreviousResultInternal());
    }

    private async navigateToPreviousResultInternal() {
        if (this.treeView.selection.length === 0) {
            if (this.provider.rootResults.length > 0) {
                this.treeView.reveal(this.provider.rootResults[this.provider.rootResults.length - 1]);
            }
        }
        else {
            let current = this.treeView.selection[0];
            switch (current.type) {
                case 'directory':
                    {
                        // select last result in previous directory 
                        if (current.prev) {
                            this.revealAndPreviewResult(await (await current.prev.children.last()?.extracts())?.last());
                        }
                        else {
                            // loop? 
                        }
                    }
                    break;
                case 'file':
                    {
                        if (current.prev) {
                            // select last result in previous file 
                            this.revealAndPreviewResult((await current.prev.extracts()).last());
                        }
                        else if (current.parent.prev) {
                            // or directory 
                            this.revealAndPreviewResult((await current.parent.prev.children.last()?.extracts())?.last());
                        }
                        else {
                            // loop? 
                        }
                    }
                    break;
                case 'extract':
                    {
                        if (current.prev) {
                            // Select previous result if possible
                            this.revealAndPreviewResult(current.prev);
                            return;
                        }
                        if (current.parent.prev) {
                            // Otherwise last result in previous file
                            this.revealAndPreviewResult((await current.parent.prev.extracts()).last());
                        }
                        else if (current.parent.parent.prev) {
                            // Otherwise last result in previous directory
                            this.revealAndPreviewResult((await current.parent.parent.prev.children.last()?.extracts())?.last());
                        }
                        else {
                            // Loop?
                        }
                    }
                    break;
            }
        }
    }

    private addNavigationOperation(operation: () => Promise<void>): Promise<void> {
        return new Promise((resolve, reject) => {
            this.currentNavOperation = this.currentNavOperation.then(operation).then(resolve).catch(reject);
        });
    }
    private revealAndPreviewResult(result: ExtractResult | undefined) {
        if (!result) { return; }
        let options = { select: true, focus: true, expand: false };
        this.treeView.reveal(result, options);
        vscode.commands.executeCommand('vscode.open', result.parent.uri, <vscode.TextDocumentShowOptions>{
            preview: true, preserveFocus: true, selection: result.range
        });
    }

    private onTreeViewSelectionChanged(event: vscode.TreeViewSelectionChangeEvent<SearchResult>) {
        // assert(event.selection.length < 2);
        // if (event.selection.length > 0) {
        //     let result = event.selection[0];
        //     switch (result.type) {
        //         case 'file':
        //             // Reveal the relevant file preview 
        //             let showOptions: vscode.TextDocumentShowOptions = { preview: true, preserveFocus: true };
        //             vscode.commands.executeCommand("vscode.open", result.uri, showOptions);
        //             this.treeView.reveal(result, { select: true, focus: true, expand: true });
        //             break;
        //     }
        // }
    }
}