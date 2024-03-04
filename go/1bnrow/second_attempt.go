package main

import (
	"1bnrow/orchestrator"
	"fmt"
	"runtime"
)


func runLoop2nd(filePath string) {
    orchestrationChannel := make(chan orchestrator.OrchestrationErrors, runtime.NumCPU())

    select {
        case msg := <- orchestrationChannel:
            fmt.Println(msg)
    }
}


