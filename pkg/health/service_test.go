package health_test

import (
	"log/slog"
	"testing"

	"github.com/proximyst/cov/pkg/health"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestService(t *testing.T) {
	t.Parallel()

	t.Run("initial state has the health-service", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), slog.Default())
		<-svc.HealthChanged()
		h := svc.Health()
		require.Len(t, h, 1, "expected one component")
		assert.Contains(t, h, "health-service", "expected health-service to be present")
	})

	t.Run("marking healthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), slog.Default())
		svc.MarkHealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h, "test-service", "expected test-service to be present")
		assert.True(t, h["test-service"].Healthy, "expected test-service to be healthy")
		assert.Equal(t, "test reason", h["test-service"].Status, "expected test-service to have the correct status")
	})

	t.Run("marking unhealthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), slog.Default())
		svc.MarkUnhealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h, "test-service", "expected test-service to be present")
		assert.False(t, h["test-service"].Healthy, "expected test-service to be unhealthy")
		assert.Equal(t, "test reason", h["test-service"].Status, "expected test-service to have the correct status")
	})

	t.Run("marking healthy after unhealthy works", func(t *testing.T) {
		t.Parallel()

		svc := health.NewService(t.Context(), slog.Default())
		svc.MarkUnhealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h := svc.Health()
		assert.Contains(t, h, "test-service", "expected test-service to be present")
		assert.False(t, h["test-service"].Healthy, "expected test-service to be unhealthy")

		svc.MarkHealthy("test-service", "test reason")
		<-svc.HealthChanged()
		h = svc.Health()
		assert.Contains(t, h, "test-service", "expected test-service to be present")
		assert.True(t, h["test-service"].Healthy, "expected test-service to be healthy")
	})
}
