import * as vscode from 'vscode';

let _logger: Logger;
export function getLogger() {
    if (_logger === undefined) {
        _logger = new Logger();
    }
    return _logger;
}

export class Logger {
    outputChannel: vscode.OutputChannel;

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('searchium');
    }

    public log(strings: TemplateStringsArray, ...insertions: any[]) {
        let s = strings.reduce((acc, cur, i) => acc + cur + (insertions[i] ? `${insertions[i]}` : ""), "");
        this.outputChannel.appendLine(s);
        console.log(s);
    }
}