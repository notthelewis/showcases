package orchestrator

import "testing"

func TestSourceRead(t *testing.T) {
    source := newSource("./example_data.txt")
    defer source.Close()

    tokensLeft, buf := source.Read() 
    if tokensLeft == false {
        t.Fatal("reported finished when just starting") 
    }

    if string(buf) != "Tokyo;10" {
        t.Fatalf("received bad data on first read. Wanted: %v. Got: %v", "Tokyo;10", string(buf)) 
    }
}

func TestSourceSetSeekPos(t *testing.T) {
    source := newSource("./example_data.txt")
    defer source.Close()

    source.SetSeekPos(9)

    tokensLeft, buf := source.Read()
    if tokensLeft == false {
        t.Fatal("reported finished when just starting") 
    }

    if string(buf) != "Tokyo;10" {
        t.Fatalf("received bad data on read at offset. Wanted: %v. Got: %v", "Tokyo;10", string(buf)) 
    }

    res := source.SetSeekPos(1000) 
    if res == false {
        t.Fatalf("able to set seek pos beyond reasonable")
    }
}

