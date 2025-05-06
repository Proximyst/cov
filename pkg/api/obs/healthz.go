package obs

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func (s *server) Healthz(c *gin.Context) {
	health := s.health.Health()
	response := HealthResponse{
		Components: make(map[string]string, len(health)),
		Status:     HealthResponseStatusHealthy,
	}
	status := http.StatusOK
	for name, component := range health {
		componentHealth := HealthResponseStatusHealthy
		if !component.Healthy {
			response.Status = HealthResponseStatusUnhealthy
			componentHealth = HealthResponseStatusUnhealthy
			status = http.StatusServiceUnavailable
		}

		response.Components[name] = string(componentHealth) + ": " + component.Status
	}

	c.JSON(status, response)
}
