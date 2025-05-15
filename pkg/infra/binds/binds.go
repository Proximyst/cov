package binds

import (
	"context"

	"github.com/alecthomas/kong"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/collectors"
)

func Context(ctx context.Context) kong.Option {
	return kong.BindTo(ctx, (*context.Context)(nil))
}

// Metrics binds the Prometheus registry to the Kong context.
// It registers the Go collector, build info collector, and process collector.
// If the registry is nil, a new registry is created. This allows to pass in one for testing.
func Metrics(registry *prometheus.Registry) []kong.Option {
	if registry == nil {
		registry = prometheus.NewRegistry()
	}
	registry.MustRegister(collectors.NewGoCollector())
	registry.MustRegister(collectors.NewBuildInfoCollector())
	registry.MustRegister(collectors.NewProcessCollector(collectors.ProcessCollectorOpts{}))
	return []kong.Option{
		kong.Bind(registry),
		kong.BindTo(registry, (*prometheus.Gatherer)(nil)),
		kong.BindTo(registry, (*prometheus.Registerer)(nil)),
	}
}
