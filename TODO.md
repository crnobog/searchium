- [x] Register and unregister open documents with server process 
- [x] Log out all received events/messages 
- [x] Navigate to file/result from search results
- [x] UI state for no index loaded 
- [x] database info sometimes not appearing 
    not cached before view is created?
- [x] Set query input box from 'search for current token'
- [x] Expand file item when selecting
- [x] Remove statusbar item 
- [x] Search code - quick pick, then open window? 
 - [x] Is it possible to contribute to the existing search view container? 
    No
- [x] Search for selected text/token under edit cursor
- [x] Enable/disable regular expression syntax
- [x] Path filter for search 
- [x] Progress for indexing 
- [x] 'Whole word' for search 
- [x] Redo search when options are toggled 
- [x] Add logging level setting 
- [x] Add line number to search results
- [x] Strip leading whitespace from search results
- [x] Keybinding for searching for token under cursor seems not to be updating UI 
    seems to be when webview is hidden in bottom panel 
- [x] Search history
- [x] Last/next search history keybinds 
- [x] Changing search options seems to be broken after using history 
- [x] Setup bundling 
- [x] Detailed index stats
- [x] Fix moving index stats window
- [x] Add buttons for prev/next search 
- [ ] Search time stats 
- [ ] Pause/resume/refresh index 
- [ ] Add tooltips to controls
- [ ] Preview document when highlighting search result 
    seems impossible with current treeview api 
- [ ] Control focusing search results based on what triggered search 
    hitting enter - focus 
    toggling options - do not focus 

- [ ] Configuration options
    - [x] Max search results 
    - [ ] Max number of extracts per file 
- [ ] Keybindings 
    - [x] Key bindings for cycling through search results 
    - [x] Key binding to navigate to search window 
    - [x] Change search settings (when focused) 

- [ ] Build own treeview to get focus events 
- [ ] Implement proposed 'FileSearchProvider' 
- [x] Search progress indicator?
    withProgress using treeview view id?
- [ ] Rewrite-modify server - token suggestions 
- [ ] Search file paths - with quick pick?
- [ ] Search file paths - side view?

# Server rewrite

https://swtch.com/~rsc/regexp/regexp4.html
