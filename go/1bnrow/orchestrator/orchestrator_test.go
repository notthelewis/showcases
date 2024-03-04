package orchestrator

import (
	"fmt"
	"runtime"
	"testing"
)

func TestOrchestratorOpenAndCloseFile(t *testing.T) {
    o := New("./example_data.txt")

    if len(o.workers) != runtime.NumCPU() {
        o.CloseFile()
        t.Fatalf("unable to create correct number of file pointers. Wanted: %v, got: %v", runtime.NumCPU() / 3, len(o.workers))
    }

    o.CloseFile()
    if len(o.workers) != 0 {
        t.Fatalf("unable to close file pointers. Wanted: 0 handles, got: %v", len(o.workers))
    }
}

func TestOrchestratorReadChunk(t *testing.T) {
    o := New("./example_data.txt")
    defer o.CloseFile()

    messages := make([]OrchestrationMessage, 0)
    go o.ReadChunk()

    for msg := range o.OrchestrationChannel {
        messages = append(messages, msg)
    }

    for _, msg := range messages {
        if msg.Error != nil {
            fmt.Printf("{ workerNum: %v, error: %v }\n", msg.workerNum, msg.Error)
        } 
    }

    for key, val := range o.store.s {
        fmt.Println(key, val)
    }
}
