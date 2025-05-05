package health

import (
	"context"
	"sync"

	"github.com/prometheus/client_golang/prometheus"
)

var _ Marker = (*Service)(nil)

type Service struct {
	metrics    *prometheus.Registry
	health     chan component
	lock       *sync.RWMutex
	components map[string]component
}

// Marker defines the contract for marking the health of components.
type Marker interface {
	MarkHealthy(svc, why string)
	MarkUnhealthy(svc, why string)
}

type component struct {
	component string
	healthy   bool
	status    string
}

func NewService(ctx context.Context, metrics *prometheus.Registry) *Service {
	updates := make(chan component, 4)
	lock := &sync.RWMutex{}
	components := make(map[string]component)
	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case comp := <-updates:
				lock.Lock()
				components[comp.component] = comp
				lock.Unlock()
			}
		}
	}()

	svc := &Service{
		metrics:    metrics,
		health:     updates,
		lock:       lock,
		components: components,
	}
	svc.MarkHealthy("health-service", "initialised")

	return svc
}

func (s *Service) MarkHealthy(svc, why string) {
	s.health <- component{
		component: svc,
		healthy:   true,
		status:    why,
	}
}

func (s *Service) MarkUnhealthy(svc, why string) {
	s.health <- component{
		component: svc,
		healthy:   false,
		status:    why,
	}
}

func (s *Service) Metrics() *prometheus.Registry {
	return s.metrics
}

func (s *Service) Health() HealthResponse {
	s.lock.RLock()
	defer s.lock.RUnlock()

	components := make(map[string]string)
	status := HealthResponseStatusOk
	for _, comp := range s.components {
		if comp.healthy {
			components[comp.component] = "healthy: " + comp.status
		} else {
			components[comp.component] = "unhealthy: " + comp.status
			status = HealthResponseStatusError
		}
	}

	return HealthResponse{
		Components: components,
		Status:     status,
	}
}
