import { PlainMessage } from "@bufbuild/protobuf";
import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import * as ipcRequests from "./ipcRequests";
import * as ipcResponses from "./ipcResponses";
import { getLogger } from "./logger";
import * as searchium_pb from "./gen/searchium_pb";
import * as path from "path";
import assert = require("assert");
import { Direction } from "@microsoft/fast-web-utilities";

export interface SearchOptions {
    query: string,
    pathFilter: string,
    matchCase: boolean,
    wholeWord: boolean,
    regex: boolean,
}

// TODO: file locations 
interface DirectoryResult {
    type: 'directory';
    name: string;
    path: string;
    parent: undefined;
    children: (DirectoryResult | FileResult)[];
}
interface FileResult {
    type: 'file';
    name: string;
    path: string;
    uri: vscode.Uri;
    parent: DirectoryResult,
    positions: PlainMessage<searchium_pb.FilePositionSpan>[];
    // TODO: cache extracts?
}
type ExtractResult = PlainMessage<searchium_pb.FileExtract> & {
    type: 'extract';
    highlights: [number, number][];
    range: vscode.Range;
    parent: FileResult;
};

type SearchResult = DirectoryResult | FileResult | ExtractResult;

// TODO: concatenate paths as we recurse 
function convertFilesystemEntries(parent: DirectoryResult | undefined, entry: PlainMessage<searchium_pb.FileSystemEntry>, parentPath?: string): (DirectoryResult | FileResult) {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.case) {
        case 'directoryEntry':
            let dir: DirectoryResult = {
                type: 'directory',
                name: entry.name,
                path: thisPath,
                parent: undefined,
                children: []
            };
            dir.children = entry.subtype.value.entries.map(
                (e) => convertFilesystemEntries(dir, e, thisPath)
            );
            return dir;
        case 'fileEntry':
            return {
                type: 'file',
                name: entry.name,
                path: thisPath,
                parent: parent!,
                uri: vscode.Uri.file(thisPath),
                positions: entry.data?.filePositionsData?.positions ?? []
            };
        case undefined: throw new Error("Unknown filesystementr type");
    }
}

function convertFileExtracts(parent: FileResult, extract: PlainMessage<searchium_pb.FileExtract>, info: PlainMessage<searchium_pb.FilePositionSpan>): ExtractResult {
    let start = info.position - extract.offset;
    let end = start + info.length;
    let range =
        new vscode.Range(extract.lineNumber, extract.columnNumber, extract.lineNumber, extract.columnNumber + end - start);
    return {
        type: "extract",
        highlights: [[start, end]],
        parent,
        range,
        ...extract,
    };
}

// TODO: Show progress bar on treeview or elsewhere if indexing is still in progress
export class SearchResultsProvider implements vscode.TreeDataProvider<SearchResult> {
    private _onDidChangeTreeData: vscode.EventEmitter<SearchResult | undefined | void> = new vscode.EventEmitter<SearchResult | undefined | void>();
    readonly onDidChangeTreeData: vscode.Event<SearchResult | undefined | void> = this._onDidChangeTreeData.event;

    currentRequestId: bigint = 0n;
    rootResults: SearchResult[] = [];

    constructor(private channel: IpcChannel) {
    }

    // TODO: Remove first layer of tree if there's only one project/directory ?
    public populate(r: ipcResponses.SearchCodeResponse) {
        if (r.requestId < this.currentRequestId) {
            return;
        }
        this.currentRequestId = r.requestId;
        this.rootResults = [];
        if (r.searchResults.subtype.case === 'directoryEntry') {
            this.rootResults = r.searchResults.subtype.value.entries.map(
                (e) => convertFilesystemEntries(undefined, e)
            );
        }
        this._onDidChangeTreeData.fire();
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
                    label: `${element.name} (${element.positions.length} results)`,
                };
                const item = new vscode.TreeItem(label, vscode.TreeItemCollapsibleState.Collapsed);
                item.resourceUri = element.uri;
                item.iconPath = vscode.ThemeIcon.File;
                return item;
            }
            case 'extract': {
                const label: vscode.TreeItemLabel = {
                    label: element.text,
                    highlights: element.highlights,
                };
                let item = new vscode.TreeItem(label);
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
            return this.rootResults;
        }

        switch (element.type) {
            case 'directory': return element.children;
            case 'extract': return undefined;
            case 'file': {
                let extracts = await this.channel.sendRequest(new ipcRequests.GetFileExtractsRequest(element.path, element.positions, 100))
                    .then((r: ipcResponses.GetFileExtractsResponse) => r.fileExtracts);
                return extracts.map((e, i) => convertFileExtracts(element, e, element.positions[i]));
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
    constructor(private provider: SearchResultsProvider, private treeView: vscode.TreeView<SearchResult>, private channel: IpcChannel) {
        treeView.onDidChangeSelection(this.onTreeViewSelectionChanged, this);
    }

    public async onQuery(options: SearchOptions | undefined) {
        if (options) {
            return await this.executeSearch(options);
        }
        else {
            getLogger().log`Undefined query string`;
        }
    }

    public executeSearch(options: Partial<SearchOptions>): Promise<void> {
        // TODO: cancel previous tasks/deal with spamming search requests
        const maxResults = vscode.workspace.getConfiguration("searchium").get<number>("maxResults", 10000);
        return this.channel.sendRequest(new ipcRequests.SearchCodeRequest({
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
                getLogger().log`Search request complete`;
                // TODO: compare num results vs max to message truncation 
                let prefix = "";
                if (r.hitCount >= maxResults) {
                    vscode.window.showInformationMessage("Search results exceeded configured maximum. Search results will be truncated.");
                    prefix = "More than ";
                }
                this.treeView.badge = { tooltip: `${prefix}${r.hitCount.toLocaleString()} search results`, value: Number(r.hitCount) };

                this.provider.populate(r);
                // TODO: Focus view 
                vscode.commands.executeCommand('searchium-results.focus');
            })
            .catch((err) => getLogger().log`Search request failed: ${err}`);
    }

    private onTreeViewSelectionChanged(event: vscode.TreeViewSelectionChangeEvent<SearchResult>) {
        assert(event.selection.length < 2);
        if (event.selection.length > 0) {
            let result = event.selection[0];
            switch (result.type) {
                case 'file':
                    // Reveal the relevant file preview 
                    let showOptions: vscode.TextDocumentShowOptions = { preview: true, preserveFocus: true };
                    vscode.commands.executeCommand("vscode.open", result.uri, showOptions);
                    this.treeView.reveal(result, { select: true, focus: true, expand: true });
                    break;
            }
        }
    }
}