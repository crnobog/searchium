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
export type Message = NoStatusMessage
    | StatusMessage
    | SetQueryMessage
    | FocusMessage
    | SetMatchCase
    | SetWholeWord
    | SetRegex;