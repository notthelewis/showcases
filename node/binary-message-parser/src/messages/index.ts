import { header } from "./header";
import { door } from "./door";
import { lights } from "./lights"

export const inbound_messages = {
    header,
    door,
    lights,
};

/** This is useful because the header does not have a message code */
export type MessagesWithoutHeader = Omit<typeof inbound_messages, "header">;

export const message_codes = new Map<number, keyof MessagesWithoutHeader>();
message_codes.set(0x0D, "door");
message_codes.set(0x0E, "lights")
