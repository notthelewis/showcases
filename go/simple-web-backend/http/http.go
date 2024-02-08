package http

import (
	"embed"
	"html/template"
	"io/fs"
	"net/http"
	"os"
	"swb/logging"
	typederrors "swb/typed-errors"
	"swb/utils"
	"time"
)

type PageInfo struct {
	// fsPath is the path to the asset on the file system
	fsPath string
	// contentType is the Content-Type field to be sent to the browser
	contentType string
	// hasTemplate dictates whether the page has an associated template. This will not be true for an image.
	hasTemplate bool
	// getDataFn is only present if hasTemplate is true
	getDataFn func(*http.Request) map[string]any
}

var (
	pages = map[string]PageInfo{
		"/": PageInfo{
			fsPath:      "site/index.html",
			contentType: "text/html",
			hasTemplate: true,
			getDataFn: func(req *http.Request) map[string]any {
				now := time.Now()

				return map[string]any{
					"time":      now.Format(time.Kitchen),
					"isTooLate": now.Hour() < 8 || now.Hour() > 18,
				}
			},
		},
		"/two-way-binding": PageInfo{
			fsPath:      "site/static/two-way-binding.html",
			contentType: "text/html",
		},
		"/static/smooches.png": PageInfo{
			fsPath:      "site/static/smooches.png",
			contentType: "image/png",
			hasTemplate: false,
		},
		"/404": PageInfo{
			fsPath:      "site/404.html",
			contentType: "text/html",
			hasTemplate: true,
			getDataFn: func(req *http.Request) map[string]any {
				return map[string]any{
					"pageName": req.URL.Path,
				}
			},
		},
		"/500": PageInfo{
			fsPath:      "site/500.html",
			contentType: "text/html",
			hasTemplate: true,
			getDataFn: func(req *http.Request) map[string]any {
				return map[string]any{
					"pageName": req.URL.Path,
				}
			},
		},
	}

	// Static site is embedded as part of the binary. This is just for convenience, so that this binary could be shipped
	// stand-alone, with zero dependencies.
	//go:embed site
	site embed.FS
)

type Server struct {
	log *logging.Logger
}

func RunHTTP(log *logging.Logger) {
	s := Server{
		log: log,
	}

	http.HandleFunc("/", s.handler)

	if err := http.ListenAndServeTLS(":8080", "server.pem", "server.key", nil); err != nil {
		log.Write(logging.CRIT, "unable to start web server with error: "+err.Error())
		os.Exit(1)
	}
}

func (s *Server) handler(w http.ResponseWriter, req *http.Request) {
	page, pageFound := pages[req.URL.Path]
	if !pageFound {
		s.handle404(w, req, req.URL.Path)
		return
	}

	w.Header().Add("Content-Type", page.contentType)

	if page.hasTemplate {
		if page.getDataFn == nil {
			s.log.Write(logging.CRIT, "No data function is present on templated page: "+req.URL.Path)
			w.WriteHeader(500)
			return
		}
		s.handleTemplatedPage(w, req, page)
		return
	}

	s.log.Write(logging.DEBUG, "handling untemplated page")
	pageData, err := fs.ReadFile(site, page.fsPath)
	if err != nil {
		s.handle500(w, req, page)
	}

	_, err = w.Write(pageData)
	if err != nil {
		s.log.Write(logging.INFO, "unable to write data to client")
		s.log.WriteHTTPRequest(req, http.StatusUnprocessableEntity)
		return
	}

	s.log.WriteHTTPRequest(req, http.StatusOK)
}

func (s *Server) handleTemplatedPage(w http.ResponseWriter, req *http.Request, page PageInfo) {
	s.log.Write(logging.DEBUG, "handling templated page")

	template, err := template.ParseFS(site, page.fsPath)
	if err != nil {
		s.handle500(w, req, page)
		return
	}

	if err := template.Execute(w, page.getDataFn(req)); err != nil {
		s.handle500(w, req, page)
		return
	}
}

func (s *Server) handle500(w http.ResponseWriter, req *http.Request, pageInfo PageInfo) {
	s.log.WriteHTTPRequest(req, http.StatusInternalServerError)
	w.WriteHeader(http.StatusInternalServerError)

	template, err := template.ParseFS(site, pages["/500"].fsPath)
	if err != nil {
		w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
		s.log.Write(logging.ALERT, err.Error())
		return
	}

	if err := template.Execute(w, pageInfo.getDataFn(req)); err != nil {
		w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
		s.log.Write(logging.ALERT, err.Error())
		return
	}
}

func (s *Server) handle404(w http.ResponseWriter, req *http.Request, pageName string) {
	s.log.WriteHTTPRequest(req, http.StatusNotFound)

	w.WriteHeader(http.StatusNotFound)

	template, err := template.ParseFS(site, pages["/404"].fsPath)
	if err != nil {
		w.Write(utils.StringToBytesUNSAFE(typederrors.Err500.Error()))
		s.log.Write(logging.ALERT, err.Error())
		return
	}

	if err := template.Execute(w, pages["/404"].getDataFn(req)); err != nil {
		w.Write(utils.StringToBytesUNSAFE(typederrors.Err404.Error()))
		s.log.Write(logging.ALERT, err.Error())
	}
}
