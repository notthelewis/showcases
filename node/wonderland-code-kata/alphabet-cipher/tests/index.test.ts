import { decodeMessage, encodeMessage, getCryptographicNonce } from "../src"

describe("getCryptographicNonce", () => {
    it("should output the correct cryptographic nonce", ()=> {
        let keyword = "keyword"
        let message = "message"

        // Length is same
        expect(getCryptographicNonce(keyword, message)).toBe(keyword)
        // Length of keyword is greater
        expect(getCryptographicNonce(keyword + "a", message)).toBe(keyword)

        // Length of keyword is shorter
        keyword = "abc"
        let expected = "abcabca"

        expect(getCryptographicNonce(keyword, message)).toBe(expected)
    })
})

describe("encodeMessage", ()=> {
    it("should encode the message properly", ()=> {
        const keyword = "scones"
        const message = "meetmebythetree"
        const expected = "egsgqwtahuiljgs"

        expect(encodeMessage(keyword, message)).toBe(expected)
    })
})

describe("decodeMessage", ()=> {
    it("should decode the message properly", ()=> {
        const keyword = "scones"
        const message = "egsgqwtahuiljgs"
        const expected = "meetmebythetree"

        expect(decodeMessage(keyword, message)).toBe(expected)
    })
})
