package logging

import (
	"io"
	"net/http"
	"strconv"
	"strings"
	"sync"
	"time"
)

// LogEntry is a single log line.
type LogEntry struct {
	sb      strings.Builder
	time    time.Time
	level   LogLevel
	message string
}

// Write sends a LogEntry to the output sink. This could be stdout or a network source.
func (entry *LogEntry) Write(out io.Writer) (int, error) {
	levelStr := entry.level.String()
	levelStrLen := len(levelStr)
	msgLen := len(entry.message)

	if entry.sb.Cap() < levelStrLen+msgLen+1 {
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
	// writer can be anything that implements io.Writer interface, i.e. stdout, stderr or a tcp socket
	writer io.Writer
	// the current log entry to write
	entry       LogEntry
	outputLevel LogLevel
}

// New creates a new threadsafe Logger. Any calls to `Write` will write formatted text to the provided writer, as long
// as the level that is chosen at start time is >= the level of the individual log. This means that production builds
// could choose to ignore debug logs, dev builds could use debug etc. Default is info.
func New(writer io.Writer, outLevel *string) Logger {
	var lvl LogLevel

	switch *outLevel {
	case "emerg":
		lvl = EMERG
	case "alert":
		lvl = ALERT
	case "crit":
		lvl = CRIT
	case "err":
		lvl = ERROR
	case "warn":
		lvl = WARN
	case "notice":
		lvl = NOTICE
	case "info":
		lvl = INFO
	case "debug":
		lvl = DEBUG
	}

	return Logger{
		mu:          sync.Mutex{},
		writer:      writer,
		outputLevel: lvl,
	}
}

func (l *Logger) Write(level LogLevel, msg string) (int, error) {
	if level > l.outputLevel {
		return 0, nil
	}

	l.mu.Lock()
	defer l.mu.Unlock()

	l.entry.time = time.Now()
	l.entry.level = level
	l.entry.message = msg

	return l.entry.Write(l.writer)
}

func (l *Logger) WriteHTTPRequest(req *http.Request, responseCode int) (int, error) {
	return l.Write(INFO, req.Method+": "+req.URL.Path+" "+strconv.Itoa(responseCode))
}
