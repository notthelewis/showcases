import { inbound_messages } from "../messages/index";

/** A buffer to store any message of a given type, alongside an offset for
 * keeping track of the progress in reading it.
 */
export type MessageBuffer = {
    /** The position to write the next byte */
    offset: number;
    /** The buffer for this message */
    buf: Buffer;
};

/** An object containing one of each message type as it's keys, and each
 * key has a MessageBuffer as it's value.
 */
export type ClientMessageBuffers = {
    [Property in keyof typeof inbound_messages]: MessageBuffer;
};

/**
 * Creates an uninitialized buffer for every message type. This function will
 * Iterate every key on every message type, calculate it's transferred size in
 * bytes, create an uninitialized MessageBuffer object of that size, then set
 * the MessageBuffer's offset to 0.
 *
 * This is so that all of the space to hold every message type is allocated 1
 * time per client, upon first connection. This reduces allocs and deletes
 * massively when compared with creating a new buffer for every message sent
 * by a client.
 *
 * When a message has finished parsing, i.e. when its offset has reached the
 * length of its buffer, set the offset to zero so that the next instance of
 * that message type simply overwrites the previous.
 *
 */
export function create_buffers(): ClientMessageBuffers {
    // @ts-ignore
    let to_return: ClientMessageBuffers = {};

    // Iterate each message type
    for (const m of Object.keys(inbound_messages)) {
        let len = 0;

        // @ts-ignore
        // Iterate each message field
        for (const p of Object.values(inbound_messages[m])) {
            // Accounts for the fields that contains an array(e.g [UINT8, 6])
            if (Array.isArray(p)) {
                for (let i = 0; i < p[1]; i++) {
                    len += p[0].len;
                }
            } else {
                //@ts-ignore
                len += p.len;
            }
        }

        // @ts-ignore
        to_return[m] = { offset: 0, buf: Buffer.allocUnsafe(len) };
    }

    return to_return;
}

/**
 * Writes a byte to a buffer at the given offset, then increments offset.
 * @param x - The value to write (UInt8)
 * @param msg - The offset and buffer to write to.
 * @returns Return whether this message has been fully read.
 * @example Write 4 bytes to a counter object
 * ```ts
 * let msg = { offset: 0, buf: Buffer.alloc(4) };
 * for (let i = 0; i <= 3; i++) {
 *     if (write_to_offset(i, msg)) {
 *         console.log(`Finished reading: ${offset} bytes`);
 *         console.log('Result', msg.buf); // => Buffer([0, 1, 2, 3]);
 *     }
 * }
 * ```
 */
export function write_at_offset(x: number, msg: MessageBuffer): boolean {
    if (msg.offset < msg.buf.length - 1) {
        msg.buf.writeUint8(x, msg.offset++);
        return false;
    }

    msg.buf.writeUint8(x, msg.offset);
    msg.offset = 0;
    return true;
}
