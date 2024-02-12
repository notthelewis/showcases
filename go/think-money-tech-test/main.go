package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"tmtt/checkout"
)

func main() {
    lineScanner := bufio.NewScanner(os.Stdin)
    scanner := checkout.New()

    checkout.PrintAllItemKeys()

    for lineScanner.Scan() {
        line := strings.Trim(lineScanner.Text(), "\n")
        if line == "END" {
            fmt.Println(scanner.GetTotalPrice())
            break
        } 

        _, err := scanner.Scan([]rune(line)[0])
        if err != nil {
            fmt.Println(err)
            continue
        }
    }
}
