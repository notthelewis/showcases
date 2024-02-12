package token

type TokenType string

type Token struct {
    Type TokenType
    Literal string
}

const (
    ILLEGAL = ""
    EOF = ""
    
    // Identifiers
    IDENT = "IDENT" // add, foobar, x, y
    INT = "INT" // 123456789

    // Operators
    ASSIGN = "="
    PLUS = "+"

    COMMA = ","
    SEMI_COLON = ";"

    L_BRACKET = "("
    R_BRACKET = ")"
    L_CURLY = "{"
    R_CURLY = "}"

    FUNCTION = "FUNCTION"
    LET = "LET"

)
