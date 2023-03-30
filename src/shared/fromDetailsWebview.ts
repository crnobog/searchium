import { Ready } from "./fromControlsWebview";

export interface ReadyMessage {
    type : 'ready';
}

export type Message = Ready;