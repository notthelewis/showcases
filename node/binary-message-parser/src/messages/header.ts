import { UINT8, UINT16_BE, UINT24_BE } from "token-types";

/* This object describes the fields required to fill up the header, alongside
 * the type of token that the Tokenizer (strtok3) should use to populate that
 * field with.
 */
export const header = {
    requester_id: [UINT8, 4],
    message_type: UINT16_BE,
    packet_length: UINT24_BE,
};
