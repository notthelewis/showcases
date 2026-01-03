
const substitutionMap: Map<string, string> = new Map();

//                        ABCDEFGHIJKLMNOPQRSTUVWXYZ
substitutionMap.set("a", "abcdefghijklmnopqrstuvwxyz")
substitutionMap.set("b", "bcdefghijklmnopqrstuvwxyza")
substitutionMap.set("c", "cdefghijklmnopqrstuvwxyzab")
substitutionMap.set("d", "defghijklmnopqrstuvwxyzabc")
substitutionMap.set("e", "efghijklmnopqrstuvwxyzabcd")
substitutionMap.set("f", "fghijklmnopqrstuvwxyzabcde")
substitutionMap.set("g", "ghijklmnopqrstuvwxyzabcdef")
substitutionMap.set("h", "hijklmnopqrstuvwxyzabcdefg")
substitutionMap.set("i", "ijklmnopqrstuvwxyzabcdefgh")
substitutionMap.set("j", "jklmnopqrstuvwxyzabcdefghi")
substitutionMap.set("k", "klmnopqrstuvwxyzabcdefghij")
substitutionMap.set("l", "lmnopqrstuvwxyzabcdefghijk")
substitutionMap.set("m", "mnopqrstuvwxyzabcdefghijkl")
substitutionMap.set("n", "nopqrstuvwxyzabcdefghijklm")
substitutionMap.set("o", "opqrstuvwxyzabcdefghijklmn")
substitutionMap.set("p", "pqrstuvwxyzabcdefghijklmno")
substitutionMap.set("q", "qrstuvwxyzabcdefghijklmnop")
substitutionMap.set("r", "rstuvwxyzabcdefghijklmnopq")
substitutionMap.set("s", "stuvwxyzabcdefghijklmnopqr")
substitutionMap.set("t", "tuvwxyzabcdefghijklmnopqrs")
substitutionMap.set("u", "uvwxyzabcdefghijklmnopqrst")
substitutionMap.set("v", "vwxyzabcdefghijklmnopqrstu")
substitutionMap.set("w", "wxyzabcdefghijklmnopqrstuv")
substitutionMap.set("x", "xyzabcdefghijklmnopqrstuvw")
substitutionMap.set("y", "yzabcdefghijklmnopqrstuvwx")
substitutionMap.set("z", "zabcdefghijklmnopqrstuvwxy")

export function encodeMessage(keyword: string, message: string): string {
    const cryptographic_nonce = getCryptographicNonce(keyword, message)

    let encoded: string = ""
    for (let i = 0; i < message.length; i++) {
        const alphabet_position = cryptographic_nonce.charCodeAt(i) - 97
        const current_line = substitutionMap.get(message[i])!
        
        encoded += current_line[alphabet_position]
    }

    return encoded
}

export function decodeMessage(keyword: string, message: string): string {
    const cryptographic_nonce = getCryptographicNonce(keyword, message)

    let decoded: string = ""
    for (let i = 0; i < message.length; i++) {
        const alphabet_position = message.charCodeAt(i) - 97
        const current_line = substitutionMap.get(cryptographic_nonce[i])!

        decoded += current_line[alphabet_position]
    }

    return decoded
}

export function getCryptographicNonce(keyword: string, message: string): string {
    if (/[^a-z]/.test(keyword)) {
        throw new Error("Message can only contain lowercase letters")
    }

    let cryptographic_nonce: string = ""

    if (keyword.length > message.length) {
        cryptographic_nonce = keyword.slice(0, message.length)
    } else if (keyword.length === message.length) {
        cryptographic_nonce = keyword
    } else {
        let current_position = 0

        for (let _ = 0; _ < message.length; _++) {
            cryptographic_nonce += keyword[current_position++]

            if (current_position >= keyword.length) {
                current_position = 0
            }
        }
    }

    return cryptographic_nonce
}

