package health_test

import (
	"testing"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/health"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestService(t *testing.T) {
	t.Parallel()

	t.Run("initial state has the health-service", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), prometheus.NewRegistry())
		<-svc.HealthChanged()
		h := svc.Health()
		require.Len(t, h.Components, 1, "expected one component")
		assert.Contains(t, h.Components, "health-service", "expected health-service to be present")
		assert.Equal(t, health.HealthResponseStatusOk, h.Status, "expected health response status to be ok")
	})

	t.Run("marking healthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), prometheus.NewRegistry())
		svc.MarkHealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h.Components, "test-service", "expected test-service to be present")
		assert.Equal(t, health.HealthResponseStatusOk, h.Status, "expected health response status to be ok")
	})

	t.Run("marking unhealthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), prometheus.NewRegistry())
		svc.MarkUnhealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h.Components, "test-service", "expected test-service to be present")
		assert.Equal(t, health.HealthResponseStatusError, h.Status, "expected health response status to be error")
	})

	t.Run("marking healthy after unhealthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), prometheus.NewRegistry())
		svc.MarkUnhealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h.Components, "test-service", "expected test-service to be present")
		assert.Equal(t, health.HealthResponseStatusError, h.Status, "expected health response status to be error")

		svc.MarkHealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h = svc.Health()
		assert.Contains(t, h.Components, "test-service", "expected test-service to be present")
		assert.Equal(t, health.HealthResponseStatusOk, h.Status, "expected health response status to be ok")
	})
}
