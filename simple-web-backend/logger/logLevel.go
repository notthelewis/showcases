package logger

type LogLevel uint8

const (
	// System is unusable
	EMERG LogLevel = iota
	// Action must be taken immediately
	ALERT
	// Critical conditions
	CRIT
	// Error conditions
	ERROR
	// Warning conditions
	WARN
	// Normal but significant conditions
	NOTICE
	// Informational messages
	INFO
	// Debug level messages
	DEBUG
)

func (L LogLevel) String() string {
	switch L {

	case EMERG:
		return "[EMERG]"

	case ALERT:
		return "[ALERT]"

	case CRIT:
		return "[CRIT]"

	case ERROR:
		return "[ERROR]"

	case WARN:
		return "[WARN]"

	case NOTICE:
		return "[NOTICE]"

	case INFO:
		return "[INFO]"

	default:
		return "[DEBUG]"
	}
}
