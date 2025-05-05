package rest

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func (s *server) Ping(c *gin.Context) {
	c.Status(http.StatusNoContent)
}
