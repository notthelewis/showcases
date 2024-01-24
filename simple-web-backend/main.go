package main

import (
	"os"
    "net/http"
	"swb/logging"
)

var log = logging.New(os.Stderr)

func main() {
    log.Write(logging.INFO, "starting web server")

    http.HandleFunc("/", getRoot)

    // http.ListenAndServe()
}

func getRoot(w http.ResponseWriter, req *http.Request) {
    w.Write([]byte(""))
	return
}
