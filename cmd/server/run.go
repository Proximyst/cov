package server

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/collectors"
	"github.com/proximyst/cov/pkg/health"
	"github.com/proximyst/cov/pkg/rest"
)

func run(ctx context.Context, healthAddr, restAddr string) error {
	metrics := prometheus.NewRegistry()
	metrics.MustRegister(collectors.NewGoCollector())
	metrics.MustRegister(collectors.NewBuildInfoCollector())
	metrics.MustRegister(collectors.NewProcessCollector(collectors.ProcessCollectorOpts{}))
	metrics.MustRegister(prometheus.NewGaugeFunc(prometheus.GaugeOpts{Name: "up", Help: "Whether the server is up."}, func() float64 {
		return 1
	}))

	ctx, cancel := context.WithCancel(ctx)
	defer cancel()
	ctx, cancelNotify := signal.NotifyContext(ctx, os.Interrupt)
	defer cancelNotify()

	failure := make(chan error)
	go func() {
		<-ctx.Done()
		failure <- nil
	}()

	healthSvc := health.NewService(ctx, metrics)
	healthServer := newHttpServer(ctx, healthAddr, health.NewRouter(prometheus.ToTransactionalGatherer(metrics), healthSvc).Handler())
	go func() {
		failure <- healthServer.ListenAndServe()
	}()

	restServer := newHttpServer(ctx, restAddr, rest.NewRouter().Handler())
	go func() {
		failure <- restServer.ListenAndServe()
	}()

	err := <-failure
	if errors.Is(err, http.ErrServerClosed) {
		return nil
	} else {
		return err
	}
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
