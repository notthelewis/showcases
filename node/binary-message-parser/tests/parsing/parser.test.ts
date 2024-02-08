import parse from "../../src/parsing/parser";
import { fromBuffer } from "strtok3";

describe("header", () => {
    it("Should parse a valid header with expected results", async () => {
        const header = Buffer.from([
            0x00,
            0x01,
            0x02,
            0x03, // requester_id
            0x00,
            0x01, // message_type
            0x00,
            0x00,
            0x00,
        ]);

        const headerTokens = fromBuffer(header);
        const parsed = await parse("header", headerTokens);
        expect(parsed.requester_id).toStrictEqual([0, 1, 2, 3]);
        expect(parsed.message_type).toBe(1);
        expect(parsed.packet_length).toBe(0);
    });
});

describe("door", () => {
    it("should parse a valid door message", async () => {
        const door = Buffer.from([
            0x00, // door_id
            0x01, // command
        ]);

        const doorTokens = fromBuffer(door);
        const parsed = await parse("door", doorTokens);
        expect(parsed.door_id).toBe(0);
        expect(parsed.command).toBe(1);
    });
});

describe("invalid", () => {
    it("Should throw an error for an invalid message type", async () => {
        const randomBytes = Buffer.allocUnsafe(10);
        const randomTokens = fromBuffer(randomBytes);

        try {
            // @ts-ignore
            await parse("invalid", randomTokens);
        } catch (e) {
            // @ts-ignore
            expect(e.message).toBe("parse::unknown_message_type::invalid");
        }
    });

    it("Should throw an error when there's not enough data", async () => {
        const h = Buffer.from([0x00]);
        const t = fromBuffer(h);
        try {
            await parse("header", t);
        } catch (e) {
            // @ts-ignore
            expect(e.message).toBe(
                "parser::header::requester_id::read_error::End-Of-Stream"
            );
        }
    });
});
