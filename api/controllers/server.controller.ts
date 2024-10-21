import type { Context } from 'jsr:@oak/oak';
import Room from '../classes/room.class.ts';
import Player from '../classes/player.class.ts';
import type { UUID } from 'node:crypto';

export default class ServerController {
    private players: Map<UUID, UUID> = new Map();
    private rooms: Map<UUID, Room> = new Map();

    public async handleConnection(ctx: Context) {
        console.log('New connection:');
        const player = new Player(ctx);
    }

    public getRoom(id: UUID): Room | null {
        return this.rooms.get(id) || null;
    }
}
