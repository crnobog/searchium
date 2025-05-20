import * as vscode from "vscode";
import { IpcChannel, isChannel } from "./ipcChannel";
import * as ipcRequests from "./ipcRequests";
import * as ipcResponses from "./ipcResponses";
import { getLogger } from "./logger";
import * as searchium_legacy from "./gen/searchium";
import * as pb from "gen/searchium/v2/searchium";
import * as path from "path";
import './extensionsMethods';
import { SearchHistory } from "./history";
import { IndexClient } from "index/indexInterface";

export interface SearchOptions {
    query: string,
    pathFilter: string,
    matchCase: boolean,
    wholeWord: boolean,
    regex: boolean,
}

export enum NavigationBehavior {
    // Pass the range of the matched text to the vscode.open command to the text is selected 
    Selection = "Selection",
    // Pass a 0-length range at the beginning of the matched text to the vscode.open command
    // so the cursor is positioned but no text is selected
    NoSelection = "NoSelection",
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
interface FileResultPosition {
    offset: number,
    length: number,
}
interface FileResult {
    type: 'file';
    name: string;
    path: string;
    uri: vscode.Uri;
    positions: FileResultPosition[];
    extracts: () => Promise<ExtractResult[]>;
    parent: DirectoryResult,
    next?: FileResult;
    prev?: FileResult;
}
type ExtractResult = {
    type: 'extract';
    highlights: [number, number][];
    parent: FileResult;
    next?: ExtractResult;
    prev?: ExtractResult;
    text: string;
    lineNumber: number;
    range: () => Promise<vscode.Range>;
};

type SearchResult = DirectoryResult | FileResult | ExtractResult;

async function getFileExtractsFromChannel(channel: IpcChannel, file: FileResult): Promise<ExtractResult[]> {
    const positions: searchium_legacy.FilePositionSpan[] = file.positions.map(p => { return { position: p.offset, length: p.length }; });
    const extracts = (await channel.sendRequest(new ipcRequests.GetFileExtractsRequest(file.path, positions, 100))
        .then((r: ipcResponses.GetFileExtractsResponse): searchium_legacy.FileExtract[] =>
            r.fileExtracts))
        .map((e, i) =>
            convertFileExtractsLegacy(file, e, positions[i]));
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

async function getFileExtractsFromClient(client: IndexClient, file: FileResult): Promise<ExtractResult[]> {
    const r = await client.getFileExtracts(file.path, file.positions.map(p => {
        return { offsetBytes: p.offset, lengthBytes: p.length };
    }), 100);
    const doc = vscode.workspace.openTextDocument(file.uri);
    const extracts = r.fileExtracts.map((extract, index): ExtractResult => {
        // TODO: file.positions seems to contain location of extracted text, not text to highlight
        const highlightInfo = file.positions[index];
        const text = extract.fullText.trimStart();
        const charsTrimmedFromFront = extract.fullText.length - text.length;
        const highlightStart = highlightInfo.offset - extract.extractOffsetBytes;
        const highlightEnd = highlightStart + highlightInfo.length;
        return {
            type: "extract",
            highlights: [[highlightStart - charsTrimmedFromFront, highlightEnd - charsTrimmedFromFront]],
            parent: file,
            text: text.trimEnd(),
            lineNumber: extract.lineNumber,
            range: async function () {
                const doc2 = await doc;
                const navStart = doc2.positionAt(highlightInfo.offset);
                const navEnd = doc2.positionAt(highlightInfo.offset + highlightInfo.length);
                return new vscode.Range(navStart, navEnd);
            }
        };
    });
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
    getFileExtracts: (file: FileResult) => Promise<ExtractResult[]>,
    parent: DirectoryResult,
    entry: searchium_legacy.FileSystemEntry,
    parentPath?: string): FileResult {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.oneofKind) {
        case 'directoryEntry':
            throw new Error("Unexpected directory entry");
        case 'fileEntry':
            {
                const file: FileResult & { _extracts: Promise<ExtractResult[]> | undefined } = {
                    type: 'file',
                    name: entry.name,
                    path: thisPath,
                    parent: parent,
                    uri: vscode.Uri.file(thisPath),
                    positions: (entry.data?.filePositionsData?.positions ?? []).map(p => { return { offset: p.position, length: p.length }; }),
                    _extracts: undefined,
                    extracts: async function (): Promise<ExtractResult[]> {
                        if (this._extracts) { this._extracts; }
                        this._extracts = getFileExtracts(this);
                        return this._extracts;
                    }
                };
                return file;
            }
        case undefined: throw new Error("Unknown filesystement type");
    }
}

function convertDirectoryResult(
    getFileExtracts: (file: FileResult) => Promise<ExtractResult[]>,
    entry: searchium_legacy.FileSystemEntry,
    parentPath?: string
): DirectoryResult {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.oneofKind) {
        case 'directoryEntry':
            {
                const dir: DirectoryResult = {
                    type: 'directory',
                    name: entry.name,
                    path: thisPath,
                    parent: undefined,
                    children: []
                };
                dir.children = entry.subtype.directoryEntry.entries.map((e) =>
                    convertFileResult(getFileExtracts, dir, e, thisPath)
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
            }
        case 'fileEntry':
            throw new Error("Unxpected file entry");
        case undefined: throw new Error("Unknown filesystement type");
    }
}

function convertFileExtractsLegacy(
    parent: FileResult,
    // Full extract text (e.g. entire line) returned from search engine
    extract: searchium_legacy.FileExtract,
    // Info about the match wtihin the file which we should highlight
    info: searchium_legacy.FilePositionSpan): ExtractResult {
    const text = extract.text.trimStart();
    // How much whitespace was trimmed from the start of the extract so we can adjust the highlight
    const trimmed = extract.text.length - text.length;
    // Start index within the extract text of the highlight
    const start = info.position - extract.offset;
    // End index within the extract text of the highlight
    const end = start + info.length;
    const range =
        new vscode.Range(extract.lineNumber, extract.columnNumber, extract.lineNumber, extract.columnNumber + end - start);
    return {
        type: "extract",
        highlights: [[start - trimmed, end - trimmed]],
        parent,
        range: async () => range,
        text: text.trimEnd(),
        lineNumber: extract.lineNumber
    };
}

// TODO: Show progress bar on treeview or elsewhere if indexing is still in progress
export class SearchResultsProvider implements vscode.TreeDataProvider<SearchResult> {
    private _onDidChangeTreeData: vscode.EventEmitter<SearchResult | undefined | void> = new vscode.EventEmitter<SearchResult | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<SearchResult | undefined | void> = this._onDidChangeTreeData.event;

    currentRequestId = 0n;
    rootResults: SearchResult[] = [];
    treeView?: vscode.TreeView<SearchResult>;
    disposables: vscode.Disposable[] = [];
    navigationBehavior: NavigationBehavior = NavigationBehavior.Selection;

    constructor(private readonly channelOrClient: IpcChannel | IndexClient) {
        this.navigationBehavior = vscode.workspace.getConfiguration("searchium").get<NavigationBehavior>("navigationBehavior", NavigationBehavior.Selection);
        getLogger().logInformation`Initial navigation behavior is now ${this.navigationBehavior}`;
        vscode.workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration("searchium.navigationBehavior")) {
                this.navigationBehavior = vscode.workspace.getConfiguration("searchium").get<NavigationBehavior>("navigationBehavior", NavigationBehavior.Selection);
                getLogger().logInformation`Navigation behavior is now ${this.navigationBehavior}`;
                this._onDidChangeTreeData.fire(undefined);
            }
        }, null, this.disposables);
    }

    // TODO: Remove first layer of tree if there's only one project/directory ?
    public populate(rootResults: DirectoryResult[], treeView: vscode.TreeView<SearchResult>): void {
        this.treeView = treeView;
        this.rootResults = rootResults;
        this._onDidChangeTreeData.fire(undefined);
    }
    public async getTreeItem(element: SearchResult): Promise<vscode.TreeItem> {
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
                const item = new vscode.TreeItem(label);
                item.description = `line ${element.lineNumber + 1}`; // convert to 1-indexed for human label
                let selection = await element.range();
                if (this.navigationBehavior === NavigationBehavior.NoSelection) {
                    selection = new vscode.Range(selection.start, selection.start);
                }
                const showOptions: vscode.TextDocumentShowOptions = { preview: false, preserveFocus: false, selection };
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
                const extracts = await element.extracts();
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
    private currentRequestId = 0n;
    constructor(
        private readonly provider: SearchResultsProvider,
        private readonly treeView: vscode.TreeView<SearchResult>,
        private readonly channelOrClient: IpcChannel | IndexClient,
        private readonly history: SearchHistory
    ) {
        treeView.onDidChangeSelection(this.onTreeViewSelectionChanged, this);
        this.currentNavOperation = Promise.resolve();
    }

    public async onQuery(options: SearchOptions | undefined, type: "history" | undefined): Promise<void> {
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
        let execute: (_progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken) => Promise<void>;
        if (isChannel(this.channelOrClient)) {
            const channel = this.channelOrClient;
            execute = async (_progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken): Promise<void> => {
                try {
                    const r = await channel.sendRequest(new ipcRequests.SearchCodeRequest({
                        searchString: options.query ?? "",
                        filePathPattern: options.pathFilter ?? "",
                        maxResults,
                        matchCase: options.matchCase ?? false,
                        matchWholeWord: options.wholeWord ?? false,
                        includeSymLinks: false,
                        regex: options.regex ?? false,
                        useRe2Engine: false
                    }));
                    if (r.requestId < this.currentRequestId) {
                        return;
                    }
                    this.currentRequestId = r.requestId;
                    getLogger().logDebug`Search request complete`;
                    let prefix = "";
                    if (r.hitCount >= maxResults) {
                        vscode.window.showInformationMessage("Search results exceeded configured maximum. Search results will be truncated.");
                        prefix = "More than ";
                    }
                    this.treeView.badge = { tooltip: `${prefix}${r.hitCount.toLocaleString()} search results`, value: Number(r.hitCount) };
                    if (r.searchResults.subtype.oneofKind === 'directoryEntry') {
                        const getFileExtracts = (file: FileResult): Promise<ExtractResult[]> => {
                            return getFileExtractsFromChannel(channel, file);
                        };
                        const rootResults = r.searchResults.subtype.directoryEntry.entries.map((e) =>
                            convertDirectoryResult(getFileExtracts, e)
                        );
                        this.provider.populate(rootResults, this.treeView);
                    }
                    else {
                        throw new Error(`Expected topmost result from server to be directory entry, got ${r.searchResults.subtype.oneofKind}`);
                    }
                    if (type !== "history") {
                        this.treeView.reveal(this.provider.rootResults[0], { select: true, focus: false, expand: false });
                    }
                }
                catch (err) {
                    getLogger().logError`Search request failed: ${err}`;
                }
            };
        }
        else {
            const client = this.channelOrClient;
            // TODO: cancellation/interrupting with new search 
            execute = async (_progress: vscode.Progress<{ increment: number, message: string }>, _token: vscode.CancellationToken): Promise<void> => {
                try {
                    const rootResults = await client.searchFileContents({
                        queryString: options.query ?? "",
                        filePathPattern: options.pathFilter ?? "",
                        maxResults,
                        matchCase: options.matchCase ?? false,
                        matchWholeWord: options.wholeWord ?? false,
                        queryType: options.regex ? pb.FileContentsQueryType.REGEX : pb.FileContentsQueryType.WILDCARD,
                    });
                    let resultCount = 0;
                    const resultMap: Map<string, DirectoryResult> = new Map();
                    for (const root of rootResults.roots) {
                        // TODO: populate results as they come in 
                        let directory = resultMap.get(root.rootPath);
                        if (!directory) {
                            directory = <DirectoryResult>{
                                name: root.rootPath,
                                path: root.rootPath,
                                children: [],
                                parent: undefined,
                                type: "directory",
                                next: undefined,
                                prev: undefined,
                            };
                            resultMap.set(root.rootPath, directory);
                        }
                        for (const result of root.hits) {
                            resultCount += result.matchSpans.length;
                            const filePath = path.join(root.rootPath, result.fileRelativePath);
                            const fileResult: FileResult & { _extracts: Promise<ExtractResult[]> | undefined } = {
                                name: result.fileRelativePath,
                                path: filePath, // TODO: Difference betewen name and path
                                parent: directory,
                                positions: result.matchSpans.map(s => { return { offset: s.offsetBytes, length: s.lengthBytes }; }),
                                type: "file",
                                uri: vscode.Uri.file(filePath),
                                prev: directory.children.last(),
                                _extracts: undefined,
                                extracts: async function (): Promise<ExtractResult[]> {
                                    if (this._extracts) { this._extracts; }
                                    this._extracts = getFileExtractsFromClient(client, this);
                                    return this._extracts;
                                },
                            };
                            if (fileResult.prev) {
                                fileResult.prev.next = fileResult;
                            }
                            directory.children.push(fileResult);
                        }
                    }
                    let prefix = "";
                    if (resultCount > maxResults) {
                        vscode.window.showInformationMessage("Search results exceeded configured maximum. Search results will be truncated.");
                        prefix = "More than ";
                    }
                    const results: DirectoryResult[] = [];
                    for (const dir of resultMap.values()) {
                        dir.prev = results.last();
                        if (dir.prev) {
                            dir.prev.next = dir;
                        }
                        results.push(dir);
                    }
                    this.treeView.badge = { tooltip: `${prefix}${resultCount.toLocaleString()} search results`, value: Number(resultCount) };
                    this.provider.populate(results, this.treeView);
                    if (type !== "history") {
                        this.treeView.reveal(this.provider.rootResults[0], { select: true, focus: false, expand: false });
                    }
                } catch (err) {
                    getLogger().logError`Search request failed: ${err}`;
                }
            };
        }
        await vscode.window.withProgress({ location: { viewId: 'searchium-results' }, cancellable: false, title: "Searching..." },
            execute);
    };

    public async navigateToNextResult(): Promise<void> {
        this.addNavigationOperation(() => this.navigateToNextResultInternal());
    }

    private async navigateToNextResultInternal(): Promise<void> {
        if (this.treeView.selection.length === 0) {
            if (this.provider.rootResults.length > 0) {
                this.treeView.reveal(this.provider.rootResults[0]);
            }
        }
        else {
            const current = this.treeView.selection[0];
            switch (current.type) {
                case 'directory':
                    {
                        // select first child 
                        await this.revealAndPreviewResult((await current.children[0].extracts())[0]);
                    }
                    break;
                case 'file':
                    {
                        await this.revealAndPreviewResult((await current.extracts())[0]);
                    }
                    break;
                case 'extract':
                    {
                        // Select next sibling or sibling of parent 
                        if (current.next) {
                            await this.revealAndPreviewResult(current.next);
                        }
                        else if (current.parent.next) {
                            await this.revealAndPreviewResult((await current.parent.next.extracts())[0]);
                        }
                        else if (current.parent.parent.next) {
                            // select next directory's first result
                            await this.revealAndPreviewResult((await current.parent.parent.next.children[0].extracts())[0]);
                        }
                        else {
                            // loop? 
                        }
                    }
                    break;
            }
        }
    }
    public async navigateToPreviousResult(): Promise<void> {
        this.addNavigationOperation(() => this.navigateToPreviousResultInternal());
    }

    private async navigateToPreviousResultInternal(): Promise<void> {
        if (this.treeView.selection.length === 0) {
            if (this.provider.rootResults.length > 0) {
                this.treeView.reveal(this.provider.rootResults[this.provider.rootResults.length - 1]);
            }
        }
        else {
            const current = this.treeView.selection[0];
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
    private async revealAndPreviewResult(result: ExtractResult | undefined): Promise<void> {
        if (!result) { return; }
        const options = { select: true, focus: true, expand: false };
        this.treeView.reveal(result, options);
        vscode.commands.executeCommand('vscode.open', result.parent.uri, <vscode.TextDocumentShowOptions>{
            preview: true, preserveFocus: true, selection: await result.range()
        });
    }

    private onTreeViewSelectionChanged(_event: vscode.TreeViewSelectionChangeEvent<SearchResult>): void {
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