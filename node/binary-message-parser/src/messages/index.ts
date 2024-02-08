import { header } from "./header";
import { door } from "./door";

export const inbound_messages = {
    header,
    door,
};

/** This is useful because the header does not have a message code */
export type MessagesWithoutHeader = Omit<typeof inbound_messages, "header">;

export const message_codes = new Map<number, keyof MessagesWithoutHeader>();
message_codes.set(0x0D, "door");
