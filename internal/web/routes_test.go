package web

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestRenderApp(t *testing.T) {
	mux := http.NewServeMux()
	RegisterRoutes(mux)

	recorder := httptest.NewRecorder()
	request := httptest.NewRequest(http.MethodGet, "/", nil)

	mux.ServeHTTP(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", recorder.Code)
	}

	body := recorder.Body.String()
	if !strings.Contains(body, "Northworth") {
		t.Fatal("expected response body to contain app name")
	}

	if !strings.Contains(body, "References") {
		t.Fatal("expected response body to contain references tab")
	}
}

func TestRenderHealth(t *testing.T) {
	mux := http.NewServeMux()
	RegisterRoutes(mux)

	recorder := httptest.NewRecorder()
	request := httptest.NewRequest(http.MethodGet, "/healthz", nil)

	mux.ServeHTTP(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", recorder.Code)
	}

	if recorder.Body.String() != "ok\n" {
		t.Fatalf("expected health body ok, got %q", recorder.Body.String())
	}
}
