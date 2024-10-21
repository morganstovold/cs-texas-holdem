import type { UUID } from "node:crypto";

export type PlayerData = {
    id: UUID;
    name: string;
    balance: number;
};