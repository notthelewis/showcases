package logging

import (
	"io"
	"strings"
	"sync"
	"time"

	"errors"
)

type LogEntry struct {
	sb      strings.Builder
	time    time.Time
	level   LogLevel
	message string
}

func (entry *LogEntry) Write(out io.Writer) (int, error) {
    levelStr := entry.level.String()
    levelStrLen := len(levelStr)
    msgLen := len(entry.message)

    if entry.sb.Cap() < levelStrLen + msgLen {
        entry.sb.Grow(levelStrLen + msgLen - entry.sb.Cap())
    }

    entry.sb.Reset()

    entry.sb.WriteString(entry.time.UTC().Format(time.DateTime))
    entry.sb.WriteRune(' ')
    entry.sb.WriteString(entry.level.String())
    entry.sb.WriteRune(' ')
    entry.sb.WriteString(entry.message)

    return out.Write([]byte(entry.sb.String()))
}

type Logger struct {
    mu sync.Mutex
    writer io.Writer
}

func New(writer io.Writer) Logger {
    return Logger{
        mu: sync.Mutex{},
        writer: writer,
    }
}

var logEntryPool = sync.Pool{
	New: func() any {
		le := LogEntry{}
		return &le
	},
}

var ErrUnableToGetPoolEntry = errors.New("FATAL - Unable to get a new log pool entry")

func (l *Logger) Write(level LogLevel, msg string) (int, error) {
	entry, ok := logEntryPool.Get().(*LogEntry)
	if !ok {
		return 0, ErrUnableToGetPoolEntry
	}

	defer logEntryPool.Put(entry)

	entry.time = time.Now()
	entry.level = level
	entry.message = msg

    l.mu.Lock()
    defer l.mu.Unlock()

    return entry.Write(l.writer)
}
