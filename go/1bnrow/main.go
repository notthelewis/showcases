package main

import (
	"bufio"
	"fmt"
	"os"
)

const (
    MAX_LINE_LENGTH = 100

    CSV_DELIM = ';'

    LINE_BREAK = '\n'
)

type StationResult struct {
    min float32
    mean float32
    max float32
}

func main() {
    file, err := os.Open("../../../1brc/measurements.txt")
    if err != nil {
        fmt.Println(err.Error())
        os.Exit(1)
    }
    defer file.Close()

    // ________________________________________________________
    // Executed in  155.37 secs    fish           external
    //    usr time  138.13 secs  278.00 micros  138.13 secs
    //    sys time    7.81 secs  666.00 micros    7.81 secs
    scanner := bufio.NewScanner(file)
    runLoopInitial(scanner)
}

