import * as vscode from 'vscode';

export class ControlsProvider implements vscode.WebviewViewProvider {
    constructor(private readonly extensionUri: vscode.Uri) { }
    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext<unknown>,
        token: vscode.CancellationToken): void | Thenable<void> {

        webviewView.webview.options = {
            enableScripts: true
        };

        webviewView.webview.html = this.getWebViewContent();
    }

    private getWebViewContent(): string {
        return /*html*/ `
          <!DOCTYPE html>
          <html lang="en">
            <head>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
              <title>Hello World!</title>
            </head>
            <body>
              <h1>Hello World!</h1>
            </body>
          </html>
        `;
    }
}