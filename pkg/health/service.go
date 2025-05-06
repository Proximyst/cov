package health

import (
	"context"
	"log/slog"
	"maps"
	"sync"
)

// Service tracks and handles the health of components in the system.
type Service struct {
	logger *slog.Logger

	componentsLock *sync.RWMutex
	health         chan Component
	components     map[string]Component

	changeLock *sync.RWMutex
	change     chan struct{}
	closed     bool
}

// Component is a single piece in the system.
type Component struct {
	Name    string
	Healthy bool

	// Status is a human-readable status message.
	// It may be used with both healthy and unhealthy components.
	Status string
}

func NewService(ctx context.Context, logger *slog.Logger) *Service {
	svc := &Service{
		logger: logger.With("service", "health"),

		health:         make(chan Component, 1),
		componentsLock: &sync.RWMutex{},
		components:     make(map[string]Component, 16),

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

// MarkHealthy updates the status of a component to healthy.
// It is safe to call this method concurrently.
// This may block if the channel is full, however pauses are unlikely to be long.
func (s *Service) MarkHealthy(svc, why string) {
	s.health <- Component{
		Name:    svc,
		Healthy: true,
		Status:  why,
	}
}

// MarkUnhealthy updates the status of a component to unhealthy.
// It is safe to call this method concurrently.
// This may block if the channel is full, however pauses are unlikely to be long.
func (s *Service) MarkUnhealthy(svc, why string) {
	s.health <- Component{
		Name:    svc,
		Healthy: false,
		Status:  why,
	}
}

// Health gets a copy of the current health of the system.
// It is safe to call this method concurrently.
// If a change was made since last read, the notification channel is rotated with a new one.
//
// The map is a dictionary from component name to its health.
func (s *Service) Health() map[string]Component {
	s.componentsLock.RLock()
	components := maps.Clone(s.components)
	s.componentsLock.RUnlock()

	if s.closed {
		s.changeLock.Lock()
		if s.closed {
			s.change = make(chan struct{})
			s.closed = false
		}
		s.changeLock.Unlock()
	}

	return components
}

func (s *Service) HealthChanged() chan struct{} {
	s.changeLock.RLock()
	defer s.changeLock.RUnlock()

	return s.change
}

func (s *Service) handleUpdate(comp Component) {
	logger := s.logger.With("component", comp.Name)

	logger.Debug("got health update",
		"healthy", comp.Healthy,
		"status", comp.Status)
	s.componentsLock.Lock()
	s.components[comp.Name] = comp
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
