package web

import (
	"embed"
	"html/template"
	"net/http"
)

//go:embed templates/*.html
var templatesFS embed.FS

var appTemplate = template.Must(template.ParseFS(templatesFS, "templates/app.html"))

type appView struct {
	Title string
	Tabs  []tabView
}

type tabView struct {
	Name   string
	Status string
}

func RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("GET /", renderApp)
	mux.HandleFunc("GET /healthz", renderHealth)
}

func renderApp(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Header().Set("Cache-Control", "no-store")

	view := appView{
		Title: "Northworth",
		Tabs: []tabView{
			{Name: "Dashboard", Status: "planned"},
			{Name: "Cash Flow", Status: "planned"},
			{Name: "Investments", Status: "planned"},
			{Name: "Strategy", Status: "planned"},
			{Name: "Property", Status: "planned"},
			{Name: "Tax", Status: "planned"},
			{Name: "References", Status: "planned"},
			{Name: "Settings", Status: "planned"},
		},
	}

	if err := appTemplate.Execute(w, view); err != nil {
		http.Error(w, "template render failed", http.StatusInternalServerError)
	}
}

func renderHealth(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok\n"))
}
