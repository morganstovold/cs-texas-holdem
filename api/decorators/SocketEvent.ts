// deno-lint-ignore-file no-explicit-any
export function SocketEvent(event: string) {
    return function (
        target: any,
        _propertyKey: string,
        descriptor: PropertyDescriptor,
    ) {
        if (!target.constructor.eventMap) {
            // deno-lint-ignore ban-types
            target.constructor.eventMap = new Map<string, Function>();
        }
        target.constructor.eventMap.set(event, descriptor.value);
    };
}

// deno-lint-ignore ban-types
export function WebSocketHandler<T extends { new (...args: any[]): {} }>(
    constructor: T,
) {
    return class extends constructor {
        socket: WebSocket | undefined;
        constructor(...args: any[]) {
            super(...args);

            const socket = this.socket;
            if (socket) {
                socket.addEventListener('message', (msgEvent: MessageEvent) => {
                    try {
                        const messageData = JSON.parse(msgEvent.data);

                        const handler = (constructor as any).eventMap.get(
                            messageData.event,
                        );
                        if (handler) {
                            handler.apply(this, [messageData.data]);
                        }
                    } catch (error) {
                        console.error('Failed to parse message data:', error);
                    }
                });
            }
        }
    };
}
