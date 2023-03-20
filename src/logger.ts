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
}