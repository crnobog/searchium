export interface NoStatusMessage {
    type: "nostatus"
}
export interface StatusMessage {
    type: "status";
    numFiles: string;
    memory: string;
    state: string;
}
export interface SetQueryMessage {
    type: "setQuery";
    query: string;
}
export interface FocusMessage {
    type: "focus"
}
export interface SetMatchCase {
    type: "setMatchCase",
    matchCase: boolean;
}
export interface SetWholeWord {
    type: "setWholeWord",
    wholeWord: boolean;
}
export interface SetRegex {
    type: "setRegex",
    regex: boolean;
}
export interface SetOptions {
    type: "options",
    matchCase: boolean,
    pathFilter: string,
    query: string,
    regex: boolean,
    wholeWord: boolean,
}
export interface SetHistoryControls {
    type: "setHistoryControls",
    prevEnabled: boolean,
    nextEnabled: boolean,
}
export type Message = NoStatusMessage
    | StatusMessage
    | SetQueryMessage
    | FocusMessage
    | SetMatchCase
    | SetWholeWord
    | SetRegex
    | SetOptions
    | SetHistoryControls;