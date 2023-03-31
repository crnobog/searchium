import * as vscode from 'vscode';

let _logger: Logger;
export function getLogger() {
    if (_logger === undefined) {
        _logger = new Logger();
    }
    return _logger;
}

type LoggingLevel = "Debug" | "Information" | "Warning" | "Error" | "None";

export class Logger {
    outputChannel: vscode.OutputChannel;
    debug: boolean = false;
    information: boolean = false;
    warning: boolean = false;
    error: boolean = false;

    constructor() {
        let config = vscode.workspace.getConfiguration("searchium");
        let level = config.get<LoggingLevel>("loggingLevel", "Warning");
        this.outputChannel = vscode.window.createOutputChannel('searchium');
        this.setLogLevel(level);

        vscode.workspace.onDidChangeConfiguration((event: vscode.ConfigurationChangeEvent) => {
            if (event.affectsConfiguration('seachium')) {
                let level = config.get<LoggingLevel>("loggingLevel", "Warning");
                this.setLogLevel(level);
            }
        });
    }

    public logDebug(strings: TemplateStringsArray, ...insertions: any[]) {
        this.logInternal(this.debug, strings, ...insertions);
    }
    public logInformation(strings: TemplateStringsArray, ...insertions: any[]) {
        this.logInternal(this.debug, strings, ...insertions);
    }
    public logWarning(strings: TemplateStringsArray, ...insertions: any[]) {
        this.logInternal(this.debug, strings, ...insertions);
    }
    public logError(strings: TemplateStringsArray, ...insertions: any[]) {
        this.logInternal(this.debug, strings, ...insertions);
    }

    private logInternal(level: boolean, strings: TemplateStringsArray, ...insertions: any[]) {
        if (!level) { return; }
        try {
            let s = "";
            for (let i = 0; i < insertions.length; ++i) {
                s += strings[i];
                try {
                    let insertion = insertions[i];
                    if (insertion instanceof Object && insertion.toString === Object.prototype.toString) {
                        s += JSON.stringify(insertion, (key, value) => {
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
            this.outputChannel.appendLine(s);
            console.log(s);
        } catch (error) {

        }
    }

    private setLogLevel(level: LoggingLevel) {
        this.debug = this.information = this.warning = this.error = false;
        switch (level) {
            case 'Debug':
                this.debug = true;
            case 'Information':
                this.information = true;
            case 'Warning':
                this.warning = true;
            case 'Error':
                this.error = true;
            case 'None':
                break;
        }
        console.log(`logging level change to ${level}`);
        this.outputChannel.appendLine(`logging level change to ${level}`);
    }
}