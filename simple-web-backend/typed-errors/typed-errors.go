package typederrors

import "errors"

var Err404 = errors.New("Page not found")
var Err500 = errors.New("Internal server error")
