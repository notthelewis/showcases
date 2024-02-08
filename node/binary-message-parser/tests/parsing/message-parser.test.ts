import { pipeline, Readable, Writable } from "stream";
import MessageParser from "../../src/parsing/message-parser";

jest.setTimeout(30000);

describe("valid", () => {
    const valid_message = Buffer.from([
        // header message
        0x00,
        0x01,
        0x02,
        0x03, // requester_id
        0x00,
        0x0d, // message_type
        0x00,
        0x00,
        0x02,
        // door message
        0x00, // door_id
        0x01, // command
    ]);

    it("Should emit valid, parsed messages when the stream sends one byte at a time", (done) => {
        class TestSource extends Readable {
            byte_counter: number = 0;
            constructor(opts?: any) {
                super(opts);
            }
            _read(size: number): void {
                if (this.byte_counter <= valid_message.length - 1) {
                    this.push(
                        Buffer.from([valid_message[this.byte_counter++]])
                    );
                } else {
                    this.push(null);
                }
            }
        }

        let messages: any = [];

        const source = new TestSource();
        const filter = new MessageParser();
        const sink = new Writable({
            objectMode: true,
            write(chunk, encoding, cb) {
                messages.push(chunk);
                cb();
            },
        });

        pipeline(source, filter, sink, (e) => {
            if (e) {
                throw e;
            }
        });

        sink.on("finish", () => {
            expect(messages[0].type).toBe("header");
            expect(messages[0].requester_id).toStrictEqual([
                0x00, 0x01, 0x02, 0x03,
            ]);
            expect(messages[1].type).toBe("door");
            expect(messages[1].command).toBe(1);
            done();
        });
    });
});
