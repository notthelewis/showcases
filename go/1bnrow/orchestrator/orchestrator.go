package orchestrator

import (
	"errors"
	"runtime"
	"strconv"
	"strings"
	"sync"
)

type OrchestrationMessage struct {
    workerNum int
    Error error
    Message string
}

type Store struct {
    valsStored int
    s map[string][]float32
    l *sync.Mutex
} 

type Orchestrator struct {
    workers []Source

    seekPos int64

    // TODO: OrchestrationMessage pool
    OrchestrationChannel chan OrchestrationMessage

    store Store
}

func New(filePath string) Orchestrator {
    o := Orchestrator{
        OrchestrationChannel: make(chan OrchestrationMessage, runtime.NumCPU()),
        store: Store{
            s: make(map[string][]float32),
            l: &sync.Mutex{},
        },
    }

    o.openFile(filePath)
    return o
}

func (o *Orchestrator) openFile(filePath string) {
    for i := 0; i < runtime.NumCPU(); i++ {
        o.workers = append(o.workers, newSource(filePath))
    }
}

func (o *Orchestrator) ReadChunk() {
    outer: for {
        wg := sync.WaitGroup{}
        wg.Add(runtime.NumCPU())

        for workerNum, worker := range o.workers {
            setSeekSucceeded := worker.SetSeekPos(o.seekPos)
            if !setSeekSucceeded {
                for i := workerNum; i < len(o.workers); i++ {
                    wg.Done()
                }
                break outer
            }

            o.seekPos++

            hasTokensLeft, buf := worker.Read()
            if !hasTokensLeft {
                // Indicate that the file has been read completely (stage 4)
                o.OrchestrationChannel <- OrchestrationMessage{
                    workerNum: workerNum,
                    Error: errors.New("end of file"),
                }

                o.parseLine(workerNum, buf, &wg)

                wg.Wait()
                close(o.OrchestrationChannel)
                break outer
            }

            o.seekPos += int64(len(buf))
            go o.parseLine(workerNum, buf, &wg)
        }
    }
}

func (o *Orchestrator) parseLine(workerNum int, buf []byte, wg *sync.WaitGroup) {
    var station string
    var measurement float32

    splstr := strings.Split(string(buf), ";")

    if len(splstr) != 2 {
        o.OrchestrationChannel <- OrchestrationMessage{
            workerNum: workerNum,
            Error: errors.New("unable to parse line " + string(buf)),
        }
        wg.Done()
        return
    }

    m, err := strconv.ParseFloat(string(splstr[1]), 32)
    if err != nil {
        o.OrchestrationChannel <- OrchestrationMessage{
            workerNum: workerNum,
            Error: err,
        }
        wg.Done()
        return
    }

    station = splstr[0]
    measurement = float32(m)

    go o.insertData(station, measurement, workerNum, wg)
}

func (o *Orchestrator) insertData(station string, measurement float32, workerNum int, wg *sync.WaitGroup) {
    o.store.l.Lock()
    val, ok := o.store.s[station]
    if !ok {
        o.store.s[station] = append(make([]float32, 0, 1), measurement)
    } else {
        o.store.s[station] = append(val, measurement)
    }
    o.store.l.Unlock()

    wg.Done()
}

func (o *Orchestrator) CloseFile() {
    for _, ptr := range o.workers {
        ptr.Close()
    }

    o.workers = nil
}
