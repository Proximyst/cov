package health

import (
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
)

//go:generate go tool oapi-codegen -config oapi-codegen.yaml openapi.yaml
//go:generate go run ../../scripts/yaml2json -i openapi.yaml -o openapi.json

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

var _ Health = (*Service)(nil)

type Health interface {
	Health() HealthResponse
}
