package main

import (
	"log"
	"net/http"
	"os"

	"github.com/sarah-oloumi/northworth/internal/web"
)

func main() {
	addr := getenv("NORTHWORTH_ADDR", "127.0.0.1:8787")

	mux := http.NewServeMux()
	web.RegisterRoutes(mux)

	log.Printf("Northworth listening on http://%s", addr)
	if err := http.ListenAndServe(addr, mux); err != nil {
		log.Fatal(err)
	}
}

func getenv(key string, fallback string) string {
	value := os.Getenv(key)
	if value == "" {
		return fallback
	}
	return value
}
