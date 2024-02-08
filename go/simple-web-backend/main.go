// This is a simple web backend which is used to showcase how I'd approach making an internal tool that could be shipped
// as a stand-alone binary to colleagues.
package main

import (
	"flag"
	"os"
	"swb/http"
	"swb/logging"
)

var (
	logLevel = flag.String("v", "info", "The verbosity level of logs. Defaults to info")
	log      logging.Logger
)

func main() {
	flag.Parse()

	log = logging.New(os.Stderr, logLevel)
	if _, err := log.Write(logging.INFO, "starting web server on port: 8080"); err != nil {
		panic("unable to start logger with error: " + err.Error())
	}

	go http.RunHTTP(&log)

	/* Here is where the actual tool would operate */

	// This is an infinite select, to ensure that the HTTP server continues to run
	select {}
}
