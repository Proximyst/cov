package obs_test

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/goccy/go-yaml"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/api/obs"
	"github.com/proximyst/cov/pkg/health"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestOpenAPIEndpoints(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	t.Run("json", func(t *testing.T) {
		t.Parallel()

		healthSvc := health.NewService(t.Context(), slog.Default())
		router := obs.NewRouter(prometheus.ToTransactionalGatherer(prometheus.NewRegistry()), healthSvc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/api/openapi.json", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body map[string]any
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal OpenAPI JSON")
	})

	t.Run("yaml", func(t *testing.T) {
		t.Parallel()

		healthSvc := health.NewService(t.Context(), slog.Default())
		router := obs.NewRouter(prometheus.ToTransactionalGatherer(prometheus.NewRegistry()), healthSvc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/api/openapi.yaml", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/yaml; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body map[string]any
		err := yaml.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal OpenAPI YAML")
	})
}

func TestMetricsEndpoint(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	registry := prometheus.NewRegistry()
	registry.MustRegister(prometheus.NewGaugeFunc(
		prometheus.GaugeOpts{
			Name: "test",
			Help: "This is a test metric.",
		},
		func() float64 {
			return 1
		}))

	healthSvc := health.NewService(t.Context(), slog.Default())
	router := obs.NewRouter(prometheus.ToTransactionalGatherer(registry), healthSvc)

	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/api/metrics", nil))

	assert.Equal(t, http.StatusOK, recorder.Code)
	assert.Equal(t, "text/plain; charset=utf-8", recorder.Header().Get("Content-Type"))
	assert.Contains(t, recorder.Body.String(), "test 1", "expected test metric to be present")
}

func TestHealthzEndpoint(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	t.Run("healthy", func(t *testing.T) {
		t.Parallel()

		healthSvc := health.NewService(t.Context(), slog.Default())
		changed := healthSvc.HealthChanged()
		healthSvc.MarkHealthy("test-service", "test reason")
		<-changed
		router := obs.NewRouter(prometheus.ToTransactionalGatherer(prometheus.NewRegistry()), healthSvc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/api/healthz", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body obs.HealthResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal health response")

		assert.Equal(t, obs.HealthResponseStatusHealthy, body.Status, "expected health response status to be healthy")
		assert.Contains(t, body.Components, "test-service", "expected test-service to be present")
	})

	t.Run("unhealthy", func(t *testing.T) {
		t.Parallel()

		healthSvc := health.NewService(t.Context(), slog.Default())
		changed := healthSvc.HealthChanged()
		healthSvc.MarkUnhealthy("test-service", "test reason")
		<-changed
		router := obs.NewRouter(prometheus.ToTransactionalGatherer(prometheus.NewRegistry()), healthSvc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/api/healthz", nil))

		assert.Equal(t, http.StatusServiceUnavailable, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body obs.HealthResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal health response")

		assert.Equal(t, obs.HealthResponseStatusUnhealthy, body.Status, "expected health response status to be unhealthy")
		assert.Contains(t, body.Components, "test-service", "expected test-service to be present")
	})
}

func TestErrorsReturnJSON(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	healthSvc := health.NewService(t.Context(), slog.Default())
	router := obs.NewRouter(prometheus.ToTransactionalGatherer(prometheus.NewRegistry()), healthSvc)

	router.GET("/test-panic", func(ctx *gin.Context) {
		panic("test panic")
	})

	t.Run("recovery middleware", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/test-panic", nil))

		assert.Equal(t, http.StatusInternalServerError, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body obs.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, obs.ErrorResponseErrorInternalServerError, body.Error, "expected error response to be internal server error")
	})

	t.Run("no route", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/this-path-doesnt-exist", nil))

		assert.Equal(t, http.StatusNotFound, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body obs.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, obs.ErrorResponseErrorNotFound, body.Error, "expected error response to be internal server error")
	})

	t.Run("method not allowed", func(t *testing.T) {
		t.Parallel()

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "POST", "/test-panic", nil))

		assert.Equal(t, http.StatusMethodNotAllowed, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body obs.ErrorResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal error response")
		assert.Equal(t, obs.ErrorResponseErrorMethodNotAllowed, body.Error, "expected error response to be internal server error")
	})
}
