package obs

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/health"
)

//go:generate go tool oapi-codegen -config oapi-codegen.yaml openapi.yaml
//go:generate go run ../../../scripts/yaml2json -i openapi.yaml -o openapi.json

type Flags struct {
	Address string `help:"The address to bind the observability server to." default:":8081"`
}

func Serve(ctx context.Context, g prometheus.Gatherer, h Health, flags Flags) error {
	router := NewRouter(prometheus.ToTransactionalGatherer(g), h)
	srv := &http.Server{
		Addr:              flags.Address,
		Handler:           router,
		ReadHeaderTimeout: 3 * time.Second,
		IdleTimeout:       30 * time.Second,
	}

	go func() {
		<-ctx.Done()
		ctx := context.WithoutCancel(ctx)
		ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
		defer cancel()
		err := srv.Shutdown(ctx)
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			slog.Error("failed to shutdown observability server", "error", err)
		}
	}()

	slog.Info("starting health server", "address", srv.Addr)
	if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		return err
	}
	return nil
}

func NewRouter(gatherer prometheus.TransactionalGatherer, health Health) *gin.Engine {
	router := gin.New()
	router.HandleMethodNotAllowed = true
	router.Use(gin.CustomRecovery(func(c *gin.Context, err any) {
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error: ErrorResponseErrorInternalServerError,
		})
		slog.Error("gin recovered from panic", "error", err)
	}))
	router.NoRoute(func(c *gin.Context) {
		c.JSON(http.StatusNotFound, ErrorResponse{
			Error: ErrorResponseErrorNotFound,
		})
	})
	router.NoMethod(func(c *gin.Context) {
		c.JSON(http.StatusMethodNotAllowed, ErrorResponse{
			Error: ErrorResponseErrorMethodNotAllowed,
		})
	})

	RegisterHandlers(router, &server{
		metrics: gatherer,
		health:  health,
	})
	return router
}

var _ ServerInterface = (*server)(nil)

type server struct {
	metrics prometheus.TransactionalGatherer
	health  Health
}

type Health interface {
	Health() map[string]health.Component
	MarkHealthy(svc, why string)
	MarkUnhealthy(svc, why string)
}
