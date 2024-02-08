import { MessagesWithoutHeader, message_codes } from "../../src/messages";
import { create_buffers } from "../../src/parsing/create-message-buffers";
import format_buffer_string from "./format-buffer-string";

function write_message_code_to_header(
    header: Buffer,
    code: number,
    code_position: number
) {
    header.writeUInt16BE(code, code_position);
}

function get_byte_0_BE(n: number): number {
    return n & 0xff;
}

function get_byte_1_BE(n: number): number {
    return (n >>> 8) & 0xff;
}

function get_byte_2_BE(n: number): number {
    return n >>> 16;
}

function write_length_to_header(
    header: Buffer,
    length: number,
    length_position: number
) {
    header.writeUInt8(get_byte_2_BE(length), length_position++);
    header.writeUInt8(get_byte_1_BE(length), length_position++);
    header.writeUInt8(get_byte_0_BE(length), length_position);
}

export default function build_header(
    m_type: keyof MessagesWithoutHeader,
    m_len: number
): Buffer {
    // Byte position of the message type code in the header
    const code_position = 4;

    // Byte position of the message length in the header
    const message_length_position = 6;

    let message_found = false;
    let header: Buffer;

    for (const [code, name] of message_codes.entries()) {
        if (name == (m_type as unknown)) {
            // Zero out uninitialized buffer
            header = create_buffers()["header"].buf.fill(0);

            write_message_code_to_header(header, code, code_position);
            write_length_to_header(header, m_len, message_length_position);

            message_found = true;
            break;
        }
    }

    if (message_found) {
        console.log(`build_header::success::${format_buffer_string(header!)}`);
        return header!;
    } else {
        throw new Error(
            `integration_tests::build_header::message_type_not_found:${String(
                m_type
            )}`
        );
    }
}
