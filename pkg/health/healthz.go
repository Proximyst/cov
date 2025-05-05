package health

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func (s *server) Healthz(c *gin.Context) {
	health := s.health.Health()
	status := http.StatusOK
	if health.Status != HealthResponseStatusOk {
		status = http.StatusServiceUnavailable
	}

	c.JSON(status, health)
}
