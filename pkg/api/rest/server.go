package rest

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/proximyst/cov/pkg/auth"
	"github.com/proximyst/cov/pkg/db"
)

//go:generate go tool oapi-codegen -config oapi-codegen.yaml openapi.yaml
//go:generate go run ../../../scripts/yaml2json -i openapi.yaml -o openapi.json

type Flags struct {
	Address string `help:"The address to bind the REST API server to." default:":8080"`
}

func Serve(ctx context.Context, pool *pgxpool.Pool, flags Flags) error {
	router, err := NewRouter(pool)
	if err != nil {
		return err
	}
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
			slog.Error("failed to shutdown rest api server", "error", err)
		}
	}()

	slog.Info("starting rest api server", "address", srv.Addr)
	if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		return err
	}
	return nil
}

func NewRouter(pool *pgxpool.Pool) (*gin.Engine, error) {
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

	enforcer, err := auth.NewEnforcer()
	if err != nil {
		return nil, err
	}
	router.Use(authorizer(enforcer, db.New(pool)))

	RegisterHandlers(router, &server{
		pool: pool,
	})
	return router, nil
}

var _ ServerInterface = (*server)(nil)

type server struct {
	pool *pgxpool.Pool
}
