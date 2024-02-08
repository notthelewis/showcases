import { inbound_messages } from "../messages";
import { BufferTokenizer } from "strtok3/lib/BufferTokenizer";

/**
 * If a message field contains an array, return an array of numbers, otherwise
 * return a single number.
 */
export type ParsedMessage<T extends keyof typeof inbound_messages> = {
    [Property in keyof typeof inbound_messages[T]]: typeof inbound_messages[T][Property] extends Array<any>
        ? Array<number>
        : number;
};

/**
 * Parse a message of a given type from a BufferTokenizer.
 * @param message_type The type of message to parse
 * @param buffer The tokenizer to parse from
 * @returns An object representing the parsed message, containing each property of a message type alongside a number or a number[].
 */
async function parse<T extends keyof typeof inbound_messages>(
    message_type: T,
    buffer: BufferTokenizer
): Promise<ParsedMessage<T>> {
    // Validate whether the message type exists
    if (!inbound_messages[message_type]) {
        throw new Error(`parse::unknown_message_type::${String(message_type)}`);
    }

    let to_return: ParsedMessage<T> = {} as ParsedMessage<T>;

    // Iterate each field of the message schema
    for await (const [key, entry] of Object.entries(
        inbound_messages[message_type]
    )) {
        try {
            // If the entry is an array, read the token at entry[0], entry[1] times
            if (Array.isArray(entry)) {
                let temp = [];
                // The @ts-ignores here are used because the object is instantiated
                // with no properties/entries.
                for (let i = 0; i < entry[1]; i++) {
                    // @ts-ignore
                    temp.push(await buffer.readToken(entry[0]));
                }
                // @ts-ignore
                to_return[key] = temp;
            } else {
                // @ts-ignore
                to_return[key] = await buffer.readToken(entry);
            }
        } catch (e) {
            throw new Error(
                // @ts-ignore
                `parser::${message_type}::${key}::read_error::${e.message}`
            );
        }
    }

    return to_return;
}

export default parse;
