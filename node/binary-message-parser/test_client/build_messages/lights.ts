import build_header from "./header";
import { create_buffers } from "../../src/parsing/create-message-buffers";
import format_buffer_string from "./format-buffer-string";

export default function build_lights(light_id: number, dimmer_value: number): Buffer {
    const light_buffer = create_buffers().lights.buf;

    light_buffer.writeUint8(light_id);
    light_buffer.writeUint8(dimmer_value, 1);

    const header = build_header("lights", light_buffer.length);
    const full_message = Buffer.concat([header, light_buffer]);
    console.log(
        `build_lights::success::${
            format_buffer_string(full_message)
        }`
    );

    return full_message;
}
