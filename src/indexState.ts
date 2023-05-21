import { IpcChannel, isChannel } from "./ipcChannel";
import { TypedEvent } from "./ipcEvents";
import * as ipcRequests from './ipcRequests';
import * as ipcResponses from './ipcResponses';
import { TypedEmitter } from "tiny-typed-emitter";
import { IndexClient, IndexStatus } from "index/indexInterface";
import { nextTick } from "process";

interface IndexStateEvents {
    'updatedLegacy': (response: ipcResponses.GetDatabaseStatisticsResponse) => void,
    'updated': (response: IndexStatus) => void,
}

// TODO: Hook other evente to send state to listeners?
export class IndexState extends TypedEmitter<IndexStateEvents> {
    constructor(private readonly clientOrChannel: IpcChannel | IndexClient) {
        super();
        if (isChannel(this.clientOrChannel)) {
            const channel = this.clientOrChannel;
            this.clientOrChannel.on('event', (event: TypedEvent) => {
                switch (event.eventType) {
                    case 'indexingServerStateChanged':
                        this.updateIndexState(channel);
                        break;
                }
            });
        }
        else {
            const client = this.clientOrChannel;
            nextTick(async (): Promise<void> => {
                for await (const status of client.getStatus()) {
                    this.emit('updated', status);
                }
            });
        }
    }

    private async updateIndexState(channel: IpcChannel): Promise<void> {
        const result = await channel.sendRequest(new ipcRequests.GetDatabaseStatisticsRequest()) as ipcResponses.GetDatabaseStatisticsResponse;
        this.emit('updatedLegacy', result);
    }
}