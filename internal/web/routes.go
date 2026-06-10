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
	Icon   string
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
			{Name: "Dashboard", Status: "planned", Icon: "layout-dashboard"},
			{Name: "Cash Flow", Status: "planned", Icon: "wallet"},
			{Name: "Investments", Status: "planned", Icon: "chart"},
			{Name: "Strategy", Status: "planned", Icon: "compass"},
			{Name: "Property", Status: "planned", Icon: "home"},
			{Name: "Tax", Status: "planned", Icon: "calculator"},
			{Name: "References", Status: "planned", Icon: "book-open"},
			{Name: "Settings", Status: "planned", Icon: "settings"},
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
