package server

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/collectors"
	"github.com/proximyst/cov/pkg/api/health"
	"github.com/proximyst/cov/pkg/api/rest"
	"github.com/proximyst/cov/pkg/db"
)

func run(ctx context.Context, healthAddr, restAddr, dbConnString string) error {
	metrics := prometheus.NewRegistry()
	metrics.MustRegister(collectors.NewGoCollector())
	metrics.MustRegister(collectors.NewBuildInfoCollector())
	metrics.MustRegister(collectors.NewProcessCollector(collectors.ProcessCollectorOpts{}))
	metrics.MustRegister(prometheus.NewGaugeFunc(prometheus.GaugeOpts{Name: "up", Help: "Whether the server is up."}, func() float64 {
		return 1
	}))

	failure := make(chan error)

	slog.Info("starting health server", "address", healthAddr)
	healthSvc := health.NewService(ctx, metrics)
	healthServer := newHttpServer(ctx, healthAddr, health.NewRouter(prometheus.ToTransactionalGatherer(metrics), healthSvc).Handler())
	go func() { failure <- healthServer.ListenAndServe() }()

	slog.Info("connecting to db")
	pool, err := db.Connect(ctx, dbConnString)
	if err != nil {
		return err
	}
	defer pool.Close()

	slog.Info("starting rest server", "address", restAddr)
	restServer := newHttpServer(ctx, restAddr, rest.NewRouter().Handler())
	go func() { failure <- restServer.ListenAndServe() }()

	slog.Info("server started", "health-address", healthAddr, "rest-address", restAddr)
	err = <-failure
	slog.Info("server shutting down")
	return err
}

func newHttpServer(ctx context.Context, addr string, handler http.Handler) *http.Server {
	server := &http.Server{
		Addr:              addr,
		Handler:           handler,
		ReadHeaderTimeout: 3 * time.Second,
		IdleTimeout:       30 * time.Second,
	}

	go func() {
		<-ctx.Done()
		ctx := context.WithoutCancel(ctx)
		ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
		defer cancel()
		err := server.Shutdown(ctx)
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			slog.Error("failed to shutdown http server", "error", err)
		}
	}()

	return server
}
