package lexer

type Lexer struct {
    input string
    pos int // Current position in input 
    readPos int // Current reading position in input
    ch byte // Current character
}


func New(input string) *Lexer {
    l := Lexer{input: input}
    return &l
}
