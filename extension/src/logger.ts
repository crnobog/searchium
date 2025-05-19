import * as vscode from 'vscode';

let _logger: Logger;
export function getLogger(): Logger {
    if (_logger === undefined) {
        _logger = new Logger();
    }
    return _logger;
}

export class Logger {
    outputChannel: vscode.LogOutputChannel;

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('searchium', { log: true });
    }

    public logDebug(strings: TemplateStringsArray, ...insertions: any[]): void {
        try {
            const s = this.logInternal(strings, ...insertions);
            this.outputChannel.debug(s);
            console.log(s);
        }
        catch { /* empty */ }
    }
    public logTrace(strings: TemplateStringsArray, ...insertions: any[]): void {
        try {
            const s = this.logInternal(strings, ...insertions);
            this.outputChannel.trace(s);
            console.log(s);
        }
        catch { /* empty */ }
    }
    public logInformation(strings: TemplateStringsArray, ...insertions: any[]): void {
        try {
            const s = this.logInternal(strings, ...insertions);
            this.outputChannel.info(s);
            console.log(s);
        }
        catch { /* empty */ }
    }
    public logWarning(strings: TemplateStringsArray, ...insertions: any[]): void {
        try {
            const s = this.logInternal(strings, ...insertions);
            this.outputChannel.warn(s);
            console.log(s);
        }
        catch { /* empty */ }
    }
    public logError(strings: TemplateStringsArray, ...insertions: any[]): void {
        try {
            const s = this.logInternal(strings, ...insertions);
            this.outputChannel.error(s);
            console.log(s);
        }
        catch { /* empty */ }
    }

    private logInternal(strings: TemplateStringsArray, ...insertions: any[]): string {
        let s = "";
        for (let i = 0; i < insertions.length; ++i) {
            s += strings[i];
            try {
                const insertion = insertions[i];
                if (insertion instanceof Object && insertion.toString === Object.prototype.toString) {
                    s += JSON.stringify(insertion, (_key, value) => {
                        if (typeof value === 'bigint') { return value.toString(); }
                        else { return value; }
                    });
                }
                else {
                    s += `${insertion}`;
                }
            } catch {
                s += "LOG_ERROR";
            }
        }
        s += strings[strings.length - 1];
        return s;
    }
}