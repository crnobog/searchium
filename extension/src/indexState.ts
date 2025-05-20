import { TypedEmitter } from "tiny-typed-emitter";
import { IndexClient, IndexStatus } from "index/indexInterface";
import { nextTick } from "process";

interface IndexStateEvents {
    'updated': (response: IndexStatus) => void,
}

// TODO: Hook other evente to send state to listeners?
export class IndexState extends TypedEmitter<IndexStateEvents> {
    constructor(private readonly client: IndexClient) {
        super();
        nextTick(async (): Promise<void> => {
            for await (const status of this.client.getStatus()) {
                this.emit('updated', status);
            }
        });
    }
}