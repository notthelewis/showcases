package lexer

import (
    "testing"
    "wat/token"
)

func TestNextToken(t *testing.T) {
    input := `=+(){};,`
    tests := []struct{
        expectedType token.TokenType
        expectedLiteral string
    } {
        {token.ASSIGN, "="},
        {token.PLUS, "+"},
        {token.L_BRACKET, "("},
        {token.R_BRACKET, ")"},
        {token.L_CURLY, "{"},
        {token.R_CURLY, "}"},
        {token.SEMI_COLON, ";"},
        {token.COMMA, ","},
    }

    l := New(input)

    for i, tt := range tests {
        tok := l.NextToken()
        if tok.Type != tt.expectedType {
            t.Fatalf(
                "test: %v failed. Expected type: %v, received: %v",
                i,
                tt.expectedType,
                tok.Type,
            )
        }
    }
}
