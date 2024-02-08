import { createServer } from "net";
import { pipeline, Writable } from "stream";

import { MessageParser } from "../parsing";

const server = createServer();

server.on("connection", (socket) => {
    // Create a new instance of the MessageParser for each inbound connection
    const parser = new MessageParser();
    // A test class, which simply logs out objects as they're coming over the
    // wire.
    const logger = new Writable({
        objectMode: true,
        write: (c, e, n) => {
            console.log(JSON.stringify(c));
            n();
        },
    });

    // Pipe each socket connection into a parser, then pipe each parser into a
    // logger for object inspection.
    pipeline(socket, parser, logger, (e) => {
        if (e) {
            console.log(`error: ${e}`);
        }
        socket.destroy();
    });
});

export default server;
