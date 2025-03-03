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
- [ ] Rewrite-modify server - token suggestions 
- [ ] Search file paths - with quick pick?
- [ ] Search file paths - side view?
- [ ] implement go to definition by heuristics 
- [ ] open search results in editor window

# Path filtering

- [ ] Include in search history
- [ ] Autocomplete?
- [ ] Change syntax to match e.g. path configuration in vs code settings 

# Server rewrite

https://swtch.com/~rsc/regexp/regexp4.html
https://en.wikipedia.org/wiki/FM-index

- [x] Progress for file contents reading
- [x] search.exclude filtering
- [x] file stats
- [x] index info/state 
- [x] case-insensitive search
- [x] regex search
- [x] search results ordering
- [x] wildcard search 
- [x] Fix off by one error in navigating to search results - something to do with range/column? 
- [ ] Default search settings are case insensitive/not whole word?
- [ ] Closing/re-opening workspaces
- [ ] Command to re-run indexing
- [ ] filesystem watching / re-indexing 
- [ ] file path filtering
- [ ] test grpc request cancellation
- [ ] Search/details/etc during indexing 
- [ ] cancel/resume indexing
    make indexing progress marker cancellable and connect that to pausing
- [ ] configure max extract len
- [ ] token suggestion
- [ ] highlighting search results in tree view seems broken
- [ ] add options to group search results by folder hierarchy? (configurable depth)
- [ ] add option to group search results by file type
- [ ] test configuring search parameters in workspace settings
- [ ] changing search parameters with workspace open
- [ ] profile initial indexing
- [ ] Use regex-syntax crate to pre-compile regexes for best implementation of whole-word test? 
- [ ] string version of search settings for vim-command like searching