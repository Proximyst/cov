package server

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/api/obs"
	"github.com/proximyst/cov/pkg/api/rest"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/health"
)

type Command struct {
	Database      db.Flags   `embed:""`
	Observability obs.Flags  `embed:"" prefix:"obs-"`
	Rest          rest.Flags `embed:"" prefix:"rest-"`
}

func (c *Command) Run(ctx context.Context, metrics *prometheus.Registry) error {
	if gin.IsDebugging() {
		gin.SetMode(gin.ReleaseMode)
	}

	metrics.MustRegister(prometheus.NewGaugeFunc(prometheus.GaugeOpts{Name: "up", Help: "Whether the server is up."}, func() float64 {
		return 1
	}))

	failure := make(chan error)
	healthSvc := health.NewService(ctx, slog.Default())
	go func() { failure <- obs.Serve(ctx, metrics, healthSvc, c.Observability) }()

	pool, err := c.Database.Connect(ctx)
	if err != nil {
		return fmt.Errorf("failed to connect to database: %w", err)
	}
	defer pool.Close()

	go func() { failure <- rest.Serve(ctx, pool, c.Rest) }()

	slog.Info("server started")
	err = <-failure
	slog.Info("server shutting down", "failure-err", err)
	return nil
}
