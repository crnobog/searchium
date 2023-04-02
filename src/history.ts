import * as vscode from "vscode";
import { getLogger } from "./logger";

const MAX_HISTORY = 100;

export class SearchHistory {
    items: string[]; // history items sorted from most recent to least recent
    index: number = 0;

    constructor(private readonly context: vscode.ExtensionContext) {
        this.items = context.workspaceState.get("SEARCHIUM_QUERY_HISTORY", []);
    }

    public clearHistory() {
        this.items = [];
        this.context.workspaceState.update("SEARCHIUM_QUERY_HISTORY", this.items);
    }

    public add(item: string) {
        this.index = 0;
        if (item === this.items[this.index]) {
            getLogger().logInformation`Skipping add to history of repeat item: '${item}'`;
            return;
        }
        getLogger().logInformation`Add to history: '${item}'`;
        this.items.unshift(item);
        if (this.items.length > MAX_HISTORY) {
            this.items.pop();
        }
        this.context.workspaceState.update("SEARCHIUM_QUERY_HISTORY", this.items);
    }

    public canNavigatePrev(): boolean {
        return this.index < this.items.length - 1;
    }
    public prev(): string | undefined {
        if (this.canNavigatePrev()) {
            this.index = ++this.index;
            let string = this.items[this.index];
            getLogger().logDebug`HistoryPrev new index ${this.index}`;
            getLogger().logDebug`History: ${this.items}`;
            return string;
        }
        return undefined;
    }

    public canNavigateNext(): boolean {
        return this.index > 0;
    }
    public next(): string | undefined {
        if (this.canNavigateNext()) {
            --this.index;
            let string = this.items[this.index];
            getLogger().logDebug`HistoryNext new index ${this.index}`;
            getLogger().logDebug`History: ${this.items}`;
            return string;
        }
        return undefined;
    }
}