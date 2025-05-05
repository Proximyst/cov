package rest_test

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/goccy/go-yaml"
	"github.com/proximyst/cov/pkg/rest"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestOpenAPIEndpoints(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	t.Run("json", func(t *testing.T) {
		t.Parallel()

		router := rest.NewRouter()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/openapi.json", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body map[string]any
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal OpenAPI JSON")
	})

	t.Run("yaml", func(t *testing.T) {
		t.Parallel()

		router := rest.NewRouter()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/openapi.yaml", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/yaml; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body map[string]any
		err := yaml.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal OpenAPI YAML")
	})
}

func TestErrorsReturnJSON(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	router := rest.NewRouter()

	router.GET("/test-panic", func(ctx *gin.Context) {
		panic("test panic")
	})

	t.Run("recovery middleware", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/test-panic", nil))

		assert.Equal(t, http.StatusInternalServerError, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body rest.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, rest.ErrorResponseErrorInternalServerError, body.Error, "expected error response to be internal server error")
	})

	t.Run("no route", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/this-path-doesnt-exist", nil))

		assert.Equal(t, http.StatusNotFound, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body rest.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, rest.ErrorResponseErrorNotFound, body.Error, "expected error response to be internal server error")
	})

	t.Run("method not allowed", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "POST", "/test-panic", nil))

		assert.Equal(t, http.StatusMethodNotAllowed, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body rest.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, rest.ErrorResponseErrorMethodNotAllowed, body.Error, "expected error response to be internal server error")
	})
}
