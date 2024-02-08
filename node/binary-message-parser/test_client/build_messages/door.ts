import build_header from "./header";
import { create_buffers } from "../../src/parsing/create-message-buffers";
import format_buffer_string from "./format-buffer-string";
export default function build_door(command_id: number): Buffer {
    const door_buffer = create_buffers().door.buf;

    const door_id = 0x00;
    const command = command_id;

    door_buffer.writeInt8(door_id);
    door_buffer.writeInt8(command, 1);

    const header = build_header("door", door_buffer.length);
    const fully_qualified_door_message = Buffer.concat([header, door_buffer]);
    console.log(
        `build_door::success::${
            format_buffer_string(fully_qualified_door_message)
        }`
    );

    return fully_qualified_door_message;
}
