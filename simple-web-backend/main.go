package main

import (
	"embed"
	"html/template"
	"net/http"
	"os"
	"swb/logging"
	typederrors "swb/typed-errors"
	"swb/utils"
	"time"
)

var (
    log = logging.New(os.Stderr)
    
    //go:embed site
    site embed.FS 

    pages = map[string]string {
        "/": "site/index.html",
        "/404": "site/404.html",
    }
) 

func main() {
    _, err := log.Write(logging.INFO, "starting web server")
    if err != nil {
        panic("unable to start logger with error: " + err.Error())
    }

    http.HandleFunc("/", handler)

    if err := http.ListenAndServe(":8080", nil); err != nil {
        log.Write(logging.CRIT, "unable to start web server with error: " + err.Error())
        return 
    }
}


func handler(w http.ResponseWriter, req *http.Request) {
    page, pageFound := pages[req.URL.Path]
    if !pageFound {
        handle404(w, req)
        return
    }

    template, err := template.ParseFS(site, page)
    if err != nil {
        w.WriteHeader(http.StatusInternalServerError)
        w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
        go log.Write(logging.ALERT, err.Error())
        return
    }

    now := time.Now()

    pageData := map[string]any{
        "time": now.Format(time.Kitchen),
        "isTooLate": now.Hour() < 8 || now.Hour() > 16,
    }

    if err := template.Execute(w, pageData); err != nil {
        w.WriteHeader(http.StatusInternalServerError)
        w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
        go log.Write(logging.ALERT, err.Error())
        return
    }

    log.Write(logging.INFO, "index.html")
}

func handle404(w http.ResponseWriter, req *http.Request) {
    w.WriteHeader(http.StatusNotFound)

    template, err := template.ParseFS(site, pages["/404"])
    if err != nil {
        w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
        go log.Write(logging.CRIT, err.Error())
        return
    }

    pageData := map[string]any {
        "pageName": req.URL.Path,
    }

    if err := template.Execute(w, pageData); err != nil {
        w.Write(utils.StringToBytesUNSAFE(typederrors.Err404.Error()))
        go log.Write(logging.ALERT, err.Error())
    }
}
