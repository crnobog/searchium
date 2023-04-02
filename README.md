# searchium README

## Features

Searchium is a port of the search engine daemon from VsChromium[https://chromium.github.io/vs-chromium/] to VS Code.

## Demo

https://user-images.githubusercontent.com/791661/229328943-034baa3c-8909-4bfe-8cca-e4d3da5668a1.mp4

## Requirements

Binaries for the search server are not currently included. They need to be installed into the 'bin' subdirectory of the extension folder.

Workspaces should contain a vs-chromium-project.txt file in the root. 

## Extension Commands

* `searchium.newSearch`: Summons a quickinput window to search for the entered string.
* `searchium.focusSearch`: Reveals the searchium controls and focuses on the search query input box. Default binding: ctrl+shift+'
* `searchium.searchCurrentToken`: Searches for the current selected text or the token under the cursor in the active text editor. Default binding: ctrl+shift+;
* `searchium.toggleCaseSensitivity`: Toggle whether searches are case sensitive or not. Default binding: alt+c with the searchium controls focused.
* `searchium.toggleWholeWord`: Toggle searching for a whole word or allowing substring matches. Default binding: alt+w with the searchium controls focused.
* `searchium.toggleRegex`: Toggle between searching plain text queries or with regular expressions. Default binding: alt+r with the searchium controls focused.
* `searchium-controls.focus` 
* `searchium-results.focus` 

## Extension Settings

* `searchium.maxResults`: Maximum number of results to return in queries.
* `searchium.loggingLevel`: Can be set to Debug, Information, Warning, Error or None to control log output from the extension.

## Known Issues

## Release Notes

### 0.0.1

No release notes yet.
