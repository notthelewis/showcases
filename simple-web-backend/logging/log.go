package logging

import (
	"io"
	"net/http"
	"strconv"
	"strings"
	"sync"
	"time"
)

// LogEntry is effectively a single log line. This object is pooled.
type LogEntry struct {
	sb      strings.Builder
	time    time.Time
	level   LogLevel
	message string
}

// Write sends a LogEntry to the output source. The output source could be stdout or a network source.
func (entry *LogEntry) Write(out io.Writer) (int, error) {
    levelStr := entry.level.String()
    levelStrLen := len(levelStr)
    msgLen := len(entry.message)

    if entry.sb.Cap() < levelStrLen + msgLen + 1 {
        entry.sb.Grow(levelStrLen + msgLen + 1 - entry.sb.Cap())
    }

    entry.sb.Reset()

    entry.sb.WriteString(entry.time.UTC().Format(time.DateTime))
    entry.sb.WriteRune(' ')
    entry.sb.WriteString(entry.level.String())
    entry.sb.WriteRune(' ')
    entry.sb.WriteString(entry.message)
    if entry.message[len(entry.message)-1] != '\n' {
        entry.sb.WriteRune('\n')
    }

    return out.Write([]byte(entry.sb.String()))
}

type Logger struct {
    // If I had more time or impetus, I'd use a lock-free data structure here 
    mu sync.Mutex
    writer io.Writer
    entry LogEntry
}

// New creates a new threadsafe Logger. 
func New(writer io.Writer) Logger {
    return Logger{
        mu: sync.Mutex{},
        writer: writer,
    }
}

func (l *Logger) Write(level LogLevel, msg string) (int, error) {
    l.mu.Lock()
    defer l.mu.Unlock()

	l.entry.time = time.Now()
	l.entry.level = level
	l.entry.message = msg

    return l.entry.Write(l.writer)
}

func (l *Logger) WriteHTTPRequest(req *http.Request, responseCode int) (int, error) {
    return l.Write(INFO, req.Method + ": " + req.URL.Path + " " + strconv.Itoa(responseCode))
}
