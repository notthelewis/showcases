package orchestrator

import (
	"bufio"
	"fmt"
	"os"
	"sync"
)

type Source struct {
    file *os.File
    scanner *bufio.Scanner
    isReading *sync.Mutex
}

func newSource(filepath string) Source {
    file, err := os.Open(filepath)
    if err != nil {
        fmt.Println(err.Error())
        os.Exit(1)
    }
    
    s := Source{
    	file:      file,
    	scanner:   bufio.NewScanner(file),
    	isReading: &sync.Mutex{},
    }

    // Define function used by the scanner type to ascertain a token (effecively readline)  
    s.scanner.Split(func(data []byte, atEOF bool) (advance int, token []byte, err error) {
        for b := 0; b < len(data); b++ {
            if data[b] == '\n' {
                return b+1, data[:b], nil
            }
        }

        if !atEOF {
            return 0, data, nil
        }

        return 0, nil, nil
    })

    return s
}

func (s *Source) SetSeekPos(newPos int64) bool {
    _, err := s.file.Seek(newPos, 0)
    return err == nil 
}

func (s *Source) Read() (bool, []byte) {
    s.isReading.Lock()
    defer s.isReading.Unlock()
    return s.scanner.Scan(), s.scanner.Bytes()
}

func (s *Source) Close() {
    s.file.Close()
    s.scanner = nil
    s.isReading = nil
}
