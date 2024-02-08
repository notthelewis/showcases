export default function format_buffer_string(buffer: Buffer) {
    let string = "";
    buffer.forEach((byte) => {
        string += `[0x${byte.toString(16).toUpperCase()}]`;
    });
    return string;
}
