import { PlainMessage } from "@bufbuild/protobuf";
import * as vscode from "vscode";
import { IpcChannel } from "./ipcChannel";
import * as ipcRequests from "./ipcRequests";
import * as ipcResponses from "./ipcResponses";
import { getLogger } from "./logger";
import * as searchium_pb from "./gen/searchium_pb";
import * as path from "path";

// TODO: file locations 
interface DirectoryResult {
    type: 'directory';
    name: string;
    path: string;
    children: (DirectoryResult | FileResult)[];
}
interface FileResult {
    type: 'file';
    name: string;
    path: string;
    positions: PlainMessage<searchium_pb.FilePositionSpan>[];
    // TODO: cache extracts?
}
type ExtractResult = PlainMessage<searchium_pb.FileExtract> & {
    type: 'extract';
    highlights: [number, number][];
};

type SearchResult = DirectoryResult | FileResult | ExtractResult;

// TODO: concatenate paths as we recurse 
function convertFilesystemEntries(entry: PlainMessage<searchium_pb.FileSystemEntry>, parentPath?: string): (DirectoryResult | FileResult) {
    const thisPath = parentPath ? path.join(parentPath, entry.name) : entry.name;
    switch (entry.subtype.case) {
        case 'directoryEntry':
            return {
                type: 'directory',
                name: entry.name,
                path: thisPath,
                children: entry.subtype.value.entries.map(
                    (e) => convertFilesystemEntries(e, thisPath)
                )
            };
        case 'fileEntry':
            return {
                type: 'file',
                name: entry.name,
                path: thisPath,
                positions: entry.data?.filePositionsData?.positions ?? []
            };
        case undefined: throw new Error("Unknown filesystementr type");
    }
}

function convertFileExtracts(extract: PlainMessage<searchium_pb.FileExtract>, info: PlainMessage<searchium_pb.FilePositionSpan>): ExtractResult {
    let start = info.position - extract.offset;
    let end = start + info.length;
    return {
        type: "extract",
        highlights: [[start, end]],
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

    public populate(r: ipcResponses.SearchCodeResponse) {
        if (r.requestId < this.currentRequestId) {
            return;
        }
        this.currentRequestId = r.requestId;
        this.rootResults = [];
        if (r.searchResults.subtype.case === 'directoryEntry') {
            this.rootResults = r.searchResults.subtype.value.entries.map(
                (e) => convertFilesystemEntries(e)
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
                item.resourceUri = vscode.Uri.file(element.path);
                item.iconPath = vscode.ThemeIcon.File;
                return item;
            }
            case 'extract': {
                const label: vscode.TreeItemLabel = {
                    label: element.text,
                    highlights: element.highlights,
                };
                return new vscode.TreeItem(label);
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
                return extracts.map((e, i) => convertFileExtracts(e, element.positions[i]));
            }
        }
    }
    // getParent?(element: SearchResult) {
    //     throw new Error('Method not implemented.');
    // }
    // public resolveTreeItem?(item: vscode.TreeItem, element: SearchResult, token: vscode.CancellationToken): vscode.ProviderResult<vscode.TreeItem> {
    //     throw new Error('Method not implemented.');
    // }
}

export class SearchManager {
    constructor(private provider: SearchResultsProvider, private treeView: vscode.TreeView<SearchResult>, private channel: IpcChannel) {
    }
    public async newSearch() {
        // TODO: cancel previous tasks/deal with spamming search requests 
        let query = await vscode.window.showInputBox({
            title: "Searchium",
            prompt: "Search term",
        });
        if (!query) { return; }
        this.channel.sendRequest(new ipcRequests.SearchCodeRequest({
            searchString: query,
            filePathPattern: "",
            maxResults: 10000,
            matchCase: false,
            matchWholeWord: false,
            includeSymLinks: false,
            regex: false,
            useRe2Engine: false
        }))
            .then((r: ipcResponses.SearchCodeResponse) => {
                getLogger().log`Search request complete`;
                // TODO: compare num results vs max to message truncation 
                this.provider.populate(r);
                // TODO: Focus view 
                vscode.commands.executeCommand('searchium-results.focus');
            })
            .catch((err) => getLogger().log`Search request failed: ${err}`);
    }
    public searchCurrentToken(editor: vscode.TextEditor, edit: vscode.TextEditorEdit) {
    }
}