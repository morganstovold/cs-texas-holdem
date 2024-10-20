import { serve } from "https://deno.land/std@0.150.0/http/server.ts";
import { Server } from "https://deno.land/x/socket_io@0.1.1/mod.ts";

const io = new Server();

async function main() {
  const controllerFiles = await Deno.readDir("./controllers");
  for await (const file of controllerFiles) {
    if (file.isFile && file.name.endsWith(".ts")) {
      const controller = await import(`./controllers/${file.name}`);
      controller.default(io);
    }
  }
}

main();

await serve(io.handler(), {
  port: 3000,
});
