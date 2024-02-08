import {
    create_buffers,
    write_at_offset,
} from "../../src/parsing/create-message-buffers";

describe("Valid tests", () => {
    let bufs = create_buffers();

    it("Should create a valid MessageBuffer object for the header", () => {
        expect(bufs.header.buf.length).toBe(9);
        expect(bufs.header.offset).toBe(0);
    });

    it("Should write bytes at the correct offset, then reset offset", () => {
        let header = bufs.header;
        for (let i = 0; i < header.buf.length; i++) {
            write_at_offset(i, header);
        }

        expect(header.offset).toBe(0);
        expect(header.buf).toStrictEqual(
            Buffer.from([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    });

    it("Should overwrite existing values at a given offset", () => {
        expect(bufs.header.buf[0]).toBe(0);
        expect(bufs.header.offset).toBe(0);

        write_at_offset(0xff, bufs.header);
        expect(bufs.header.buf[0]).toBe(0xff);
        expect(bufs.header.offset).toBe(1);
    });
});
