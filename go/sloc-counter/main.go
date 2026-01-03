package main

import (
	"flag"
	"os"
	"strings"
)

func main() {
    flag.Parse()
    // fmt.Println(flag.Args())

    // Take args from stdin
    // For each arg:
    //  - open file
    //  - count each line - count each code line
    //  - count each comment line
    //
    // When all files are done, tally the total
}

type LineCounter struct {
    f *os.File
    totalLines uint
    sloc uint
    slComment uint
    mlCommentLines uint
    mlCommentBlocks uint
}

func (lc *LineCounter) Reset(f *os.File) {
    lc.f = f
    lc.totalLines = 0 
    lc.sloc = 0
    lc.slComment = 0
    lc.mlCommentLines = 0 
    lc.mlCommentBlocks = 0
}

type LineCount struct {
    found bool
    mlStarted bool
    mlEnded bool
}

// func (l *LineCounter) isCodeLine(line string, lc *LineCount) {
//     slStart := strings.Index(line, "//")
//     mlStart := strings.Index(line, "/*")
//     mlEnd := strings.Index(line, "*/")
//
//     lc.found = slStart != -1
//
//     slFound := slStart != -1
//     mlStartFound := mlStart != -1
//     mlEndFound := mlEnd != -1
//
//     if slFound && mlStartFound {
//         slFound = slStart < mlStart
//         mlStartFound = !slFound
//     }
//
//     if slFound && mlEndFound {
//         slFound = slStart < mlEnd
//         mlEndFound = !slFound
//     }
//
//
//     return {
//         found: slFound || mlStartFound || mlEndFound,
//         mlStarted: mlStartFound,
//         mlEnded: mlEndFound,
//     }
// }
