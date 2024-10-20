import type { Server } from "https://deno.land/x/socket_io@0.1.1/mod.ts";

export default function useRoomController(io: Server) {
    console.log("Room controller loaded");
    io.on("connection", (socket) => {
        
    })
}