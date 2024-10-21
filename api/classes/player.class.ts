import type { Context } from 'jsr:@oak/oak';
import type { PlayerData } from '../types/index.d.ts';
import { SocketEvent, WebSocketHandler } from '../decorators/SocketEvent.ts';
import { randomUUID } from "node:crypto";

@WebSocketHandler
export default class Player {
    private socket: WebSocket;
    public data: PlayerData;

    constructor(ctx: Context) {
        this.socket = ctx.upgrade();
        this.data = this.RetrievePlayerData();

        this.socket.onclose = this.onclose;
        this.socket.onerror = this.onerror;
    }

    @SocketEvent('message')
    onmessage(eventData: any) {
        console.log('Received event data:', eventData);
    }

    private onclose() {}
    private onerror() {}

    private RetrievePlayerData(): PlayerData {
        return {
            id: randomUUID(),
            name: 'Player 1',
            balance: 1000,
        };
    }
}
