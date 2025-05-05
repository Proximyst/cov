package health

import (
	"context"
	"log/slog"
	"sync"

	"github.com/prometheus/client_golang/prometheus"
)

var _ Marker = (*Service)(nil)

type Service struct {
	metrics *prometheus.Registry

	componentsLock *sync.RWMutex
	health         chan component
	components     map[string]component

	changeLock *sync.RWMutex
	change     chan struct{}
	closed     bool
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
	svc := &Service{
		metrics: metrics,

		health:         make(chan component, 1),
		componentsLock: &sync.RWMutex{},
		components:     make(map[string]component, 16),

		changeLock: &sync.RWMutex{},
		change:     make(chan struct{}),
	}
	svc.MarkHealthy("health-service", "initialised")

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case comp := <-svc.health:
				svc.handleUpdate(comp)
			}
		}
	}()

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
	s.componentsLock.RLock()
	defer s.componentsLock.RUnlock()

	components := make(map[string]string, len(s.components))
	status := HealthResponseStatusOk
	for _, comp := range s.components {
		if comp.healthy {
			components[comp.component] = "healthy: " + comp.status
		} else {
			components[comp.component] = "unhealthy: " + comp.status
			status = HealthResponseStatusError
		}
	}

	if s.closed {
		s.changeLock.Lock()
		if s.closed {
			s.change = make(chan struct{})
			s.closed = false
		}
		s.changeLock.Unlock()
	}

	return HealthResponse{
		Components: components,
		Status:     status,
	}
}

func (s *Service) HealthChanged() chan struct{} {
	s.changeLock.RLock()
	defer s.changeLock.RUnlock()

	return s.change
}

func (s *Service) handleUpdate(comp component) {
	logger := slog.With("service", "health", "component", comp.component)

	logger.Debug("got health update",
		"healthy", comp.healthy,
		"status", comp.status)
	s.componentsLock.Lock()
	s.components[comp.component] = comp
	s.componentsLock.Unlock()

	if !s.closed {
		s.changeLock.Lock()
		if !s.closed {
			close(s.change)
		}
		s.closed = true
		s.changeLock.Unlock()
	}
	logger.Debug("processed health update")
}
