import * as vscode from "vscode";
import { getLogger } from "./logger";

const MAX_HISTORY = 100;

export class SearchHistory {
    items: string[]; // history items sorted from most recent to least recent
    index: number = 0;

    constructor(private readonly context: vscode.ExtensionContext) {
        this.items = []; // context.workspaceState.get("SEARCHIUM_QUERY_HISTORY", []);
    }

    public add(item: string) {
        getLogger().logInformation`Add to history: '${item}'`;
        this.index = 0;
        this.items.unshift(item);
        if (this.items.length > MAX_HISTORY) {
            this.items.pop();
        }
        this.context.workspaceState.update("SEARCHIUM_QUERY_HISTORY", this.items);
    }

    public prev(): string | undefined {
        if (this.index < this.items.length) {
            ++this.index;
            let string = this.items[this.index];
            getLogger().logDebug`HistoryPrev new index ${this.index}`;
            return string;
        }
        return undefined;
    }
    public next(): string | undefined {
        if (this.index > 0) {
            --this.index;
            let string = this.items[this.index];
            getLogger().logDebug`HistoryNext new index ${this.index}`;
            return string;
        }
        return undefined;
    }
}