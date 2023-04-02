# searchium README

## Features

Searchium is a port of the search engine daemon from VsChromium[https://chromium.github.io/vs-chromium/] to VS Code.

## Demo

<video src="https://user-images.githubusercontent.com/791661/229328943-034baa3c-8909-4bfe-8cca-e4d3da5668a1.mp4"></video>

## Requirements

Workspaces should contain a vs-chromium-project.txt file in the root. See the VsChromium documentation for the format.

## Extension Commands

* `searchium.newSearch`: Summons a quickinput window to search for the entered string.
* `searchium.focusSearch`: Reveals the searchium controls and focuses on the search query input box. Default binding: ctrl+shift+'
* `searchium.searchCurrentToken`: Searches for the current selected text or the token under the cursor in the active text editor. Default binding: ctrl+shift+;
* `searchium.toggleCaseSensitivity`: Toggle whether searches are case sensitive or not. Default binding: alt+c with the searchium controls focused.
* `searchium.toggleWholeWord`: Toggle searching for a whole word or allowing substring matches. Default binding: alt+w with the searchium controls focused.
* `searchium.toggleRegex`: Toggle between searching plain text queries or with regular expressions. Default binding: alt+r with the searchium controls focused.
* `searchium.openDetails`: Open a window with details about the search index.
* `searchium.clearHistory`: Clear search history
* `searchium.nextResult`: Focus the next result in the results view and open the result in a preview editor.
* `searchium.previousResult`: Focus the previous result in the results view and open the result in a preview editor.
* `searchium.previousQuery`: Move one step backwards in the search history and repeat that search.
* `searchium.nextQuery`: Move one step forwards in the search history and repeat that search.
* `searchium-controls.focus` 
* `searchium-results.focus` 

## Extension Settings

* `searchium.maxResults`: Maximum number of results to return in queries.
* `searchium.loggingLevel`: Can be set to Debug, Information, Warning, Error or None to control log output from the extension.

## Building 

Binaries for the search server are not submitted to github. They need to be placed into the 'bin' subdirectory of the repository.

## Known Issues

## Release Notes

### 0.1.0

Initial published release.
