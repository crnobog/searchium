import { IpcChannel } from "./ipcChannel";
import { TypedEvent } from "./ipcEvents";
import * as ipcRequests from './ipcRequests';
import * as ipcResponses from './ipcResponses';
import { TypedEmitter } from "tiny-typed-emitter";

interface IndexStateEvents {
    'updated': (response: ipcResponses.GetDatabaseStatisticsResponse) => void,
}

// TODO: Hook other evente to send state to listeners?
export class IndexState extends TypedEmitter<IndexStateEvents> {
    constructor(private readonly channel: IpcChannel) {
        super();
        channel.on('event', this.onEvent.bind(this));
    }

    private onEvent(event: TypedEvent) {
        switch (event.eventType) {
            case 'indexingServerStateChanged':
                this.updateIndexState();
                break;
        }
    }

    private async updateIndexState() {
        let result = await this.channel.sendRequest(new ipcRequests.GetDatabaseStatisticsRequest()) as ipcResponses.GetDatabaseStatisticsResponse;
        this.emit('updated', result);
    }
}