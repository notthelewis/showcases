import { createConnection, createServer } from "net";
import { build_door, build_lights } from "./build_messages";

const door_codes = {
    open_door: 0x01,
    close_door: 0x02,
    lock_door: 0x03,
    unlock_door: 0x04,
};

const connection = createConnection({ port: 9999 }, () => {
    connection.write(build_door(door_codes.unlock_door));
    connection.write(build_lights(1, 10));
    connection.end();
});
