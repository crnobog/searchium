export interface Ready {
    command: "ready";
}
export interface QueryChanged {
    command: "setQuery";
    text: string;
}
export interface FilterChanged {
    command: "setFilter";
    text: string;
}
export interface MatchCaseChanged {
    command: "setMatchCase";
    value: boolean;
}
export interface WholeWordChanged {
    command: "setWholeWord";
    value: boolean;
}
export interface RegexChanged {
    command: "setRegex";
    value: boolean;
}
export interface Execute {
    command: "execute";
    query: string;
    pathFilter: string;
    matchCase: boolean;
    wholeWord: boolean;
    regex: boolean;
}
export type Message = Ready
    | QueryChanged
    | FilterChanged
    | MatchCaseChanged
    | WholeWordChanged
    | RegexChanged
    | Execute;