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
- [x] Default search settings are case insensitive/not whole word?
- [x] highlighting search results in tree view seems broken
    - [x] Add tests for getting file extracts to search_engine.rs
    - [x] Rename result/extract proto message names for clarity 
- [x] Option for selecting match or not when navigating
- [x] Remove legacy protobuf code
- [x] Fix eslint config 
- [x] Reorganize protobuf definitions
- [x] Server process living past test lifetime 
- [x] Searching in workspaces with multiple folders
- [x] Adding/removing folders to/from workspace
- [ ] Closing/re-opening workspaces
- [ ] filesystem watching / re-indexing 
- [ ] file path filtering
- [ ] Cancel/resume indexing
    make indexing progress marker cancellable and connect that to pausing
- [ ] Command to re-run indexing
- [ ] Profile initial indexing
- [ ] Test grpc request cancellation
- [ ] Search/details/etc during indexing 
- [ ] Configure max extract len
- [ ] Token suggestion
- [ ] Add options to group search results by folder hierarchy? (configurable depth)
- [ ] Add option to group search results by file type
- [ ] Test configuring search parameters in workspace settings
- [ ] Changing search parameters with workspace open
- [ ] Use regex-syntax crate to pre-compile regexes for best implementation of whole-word test? 
- [ ] String version of search settings for vim-command like searching
- [ ] Intercept edit events from editor and re-run search? Or use file watcher to update most recent search on open stream?

# Bugs 

- [ ] Editor state changing from "open folder" to "workspace with multiple folders" seems to break searching. Also from closing & reopening workspace?
    - "Element with id {} is already registered"
- [ ] Incorrect extracts on last line of documents
- [ ] Incorrect navigation in AK\FloatingPointStringConversions.cpp searching for log2, though highlight is correct. Caused by unicode characters earlier in file, vs code positionAt does not account for them. 