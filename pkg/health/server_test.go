package health_test

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/health"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

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

	svc := health.NewService(t.Context(), registry)
	router := health.NewRouter(prometheus.ToTransactionalGatherer(registry), svc)

	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/metrics", nil))

	assert.Equal(t, http.StatusOK, recorder.Code)
	assert.Equal(t, "text/plain; charset=utf-8", recorder.Header().Get("Content-Type"))
	assert.Contains(t, recorder.Body.String(), "test 1", "expected test metric to be present")
}

func TestHealthzEndpoint(t *testing.T) {
	t.Parallel()
	gin.SetMode(gin.TestMode)

	t.Run("healthy", func(t *testing.T) {
		t.Parallel()

		registry := prometheus.NewRegistry()
		svc := health.NewService(t.Context(), registry)
		changed := svc.HealthChanged()
		svc.MarkHealthy("test-service", "test reason")
		<-changed
		router := health.NewRouter(prometheus.ToTransactionalGatherer(registry), svc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/healthz", nil))

		assert.Equal(t, http.StatusOK, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body health.HealthResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal health response")

		assert.Equal(t, health.HealthResponseStatusOk, body.Status, "expected health response status to be ok")
		assert.Contains(t, body.Components, "test-service", "expected test-service to be present")
	})

	t.Run("unhealthy", func(t *testing.T) {
		t.Parallel()

		registry := prometheus.NewRegistry()
		svc := health.NewService(t.Context(), registry)
		changed := svc.HealthChanged()
		svc.MarkUnhealthy("test-service", "test reason")
		<-changed
		router := health.NewRouter(prometheus.ToTransactionalGatherer(registry), svc)

		recorder := httptest.NewRecorder()
		router.ServeHTTP(recorder, httptest.NewRequestWithContext(t.Context(), "GET", "/healthz", nil))

		assert.Equal(t, http.StatusServiceUnavailable, recorder.Code)
		assert.Equal(t, "application/json; charset=utf-8", recorder.Header().Get("Content-Type"))

		var body health.HealthResponse
		err := json.Unmarshal(recorder.Body.Bytes(), &body)
		require.NoError(t, err, "failed to unmarshal health response")

		assert.Equal(t, health.HealthResponseStatusError, body.Status, "expected health response status to be ok")
		assert.Contains(t, body.Components, "test-service", "expected test-service to be present")
	})
}
