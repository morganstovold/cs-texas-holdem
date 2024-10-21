import { Application, Context, Router } from 'jsr:@oak/oak';
import ServerController from './controllers/server.controller.ts';

const app = new Application();
const port = 8080;
const router = new Router();

const server = new ServerController();

router.get('/ws', (ctx: Context) => server.handleConnection(ctx));

app.use(router.routes());
app.use(router.allowedMethods());

console.log(`Listening at http://localhost:${port}`);
await app.listen({ port });
