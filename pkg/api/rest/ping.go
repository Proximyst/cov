package rest

import (
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (s *server) Ping(c *gin.Context) {
	usr, ok := userFromContext(c)
	if ok {
		slog.Info("got user from context", "user", usr.Username)
	}

	c.Status(http.StatusNoContent)
}
