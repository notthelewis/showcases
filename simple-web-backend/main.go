package main

import (
	"os"
    "net/http"
	"swb/logger"
)

var log logger.Logger 

func main() {
    log = logger.New(os.Stderr)
    log.Write(logger.INFO, "starting web server")

    http.HandleFunc("/", getRoot)

    // http.ListenAndServe()

}

func getRoot(w http.ResponseWriter, req *http.Request) {
    w.Write([]byte(""))
	return
}
