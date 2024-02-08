// external imports
import { Transform, TransformOptions, TransformCallback } from "stream";
import { fromBuffer } from "strtok3";

// internal imports
import parse from "./parser";
import { inbound_messages, message_codes } from "../messages";
import {
    write_at_offset,
    create_buffers,
    ClientMessageBuffers,
} from "./create-message-buffers";

/**
 * Takes in a readable stream of bytes and outputs parsed messages to an object
 * stream.
 * */
export default class MessageParser extends Transform {
    /** The current message type that is being read.*/
    private state: keyof typeof inbound_messages = "header";
    private m_buffers: ClientMessageBuffers = create_buffers();

    /** Set readableObjectMode, so that objects are emitted on the
     * readable side of the Duplex stream.
     */
    constructor(opts?: TransformOptions) {
        super({ ...opts, readableObjectMode: true });
    }

    /**
     * Iterate every byte in a chunk sequentially, passing each byte into the
     * read_byte class method and awaiting it's response. If the processing
     * returns a result other than null, push that result to any consumers.
     * Once the chunk has been consumed, request the next chunk using the
     * callback.
     */
    async _transform(
        chunk: Buffer,
        encoding: BufferEncoding,
        callback: TransformCallback
    ): Promise<void> {
        for await (const byte of chunk) {
            let prev_state = this.state;
            const res = await this.read_byte(byte);
            if (res != null) {
                this.push({ type: prev_state, ...res });
            }
        }
        callback();
    }

    /**
     * Write the byte that's been read into the appropriate buffer. Check
     * whether that write concludes the space for that message type.
     *
     * If it hasn't, return null.
     * If it has, parse the buffer, then return the object and update state
     * accordingly.
     */
    async read_byte(byte: number) {
        try {
            // If the byte doesn't complete the message
            if (!write_at_offset(byte, this.m_buffers[this.state])) {
                return null;
            }

            let tokenizer = fromBuffer(this.m_buffers[this.state].buf);

            /**
             * Once a message is filled, check what state we're in. If in header
             * state, determine what the type of the message is. Every other
             * message state reverts back to header state when completed, as the
             * header always prepends every message.
             */
            if (this.state == "header") {
                let parsed = await parse<"header">(this.state, tokenizer);
                let next_state = message_codes.get(parsed.message_type);

                if (typeof next_state === "undefined") {
                    throw new Error(
                        `parser::invalid_message_type::${parsed.message_type}`
                    );
                }
                this.state = next_state;
                return parsed;
            } else {
                let parsed = await parse(this.state, tokenizer);
                this.state = "header";
                return parsed;
            }
        } catch (e) {
            throw e;
        }
    }

    /** Mark all allocated buffers for deletion. */
    _destroy(
        error: Error | null,
        callback: (error: Error | null) => void
    ): void {
        // @ts-ignore
        delete this.m_buffers;
        // @ts-ignore
        delete this;
    }
}
