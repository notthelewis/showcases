package typederrors

import "errors"

var ErrUnableToGetPoolEntry = errors.New("FATAL - Unable to get a new log pool entry")

var Err404 = errors.New("Page not found")
var Err500 = errors.New("Internal server error")
