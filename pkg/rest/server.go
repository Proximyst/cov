package rest

import (
	"log/slog"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

func Start() {
	gin.SetMode(gin.ReleaseMode)

	router := gin.New()
	router.Use(gin.CustomRecovery(func(c *gin.Context, err any) {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": "internal server error",
		})
		slog.Error("gin recovered from panic", "error", err)
	}))
	router.NoRoute(func(c *gin.Context) {
		c.JSON(http.StatusNotFound, gin.H{
			"error": "route not found",
		})
	})
	router.NoMethod(func(c *gin.Context) {
		c.JSON(http.StatusMethodNotAllowed, gin.H{
			"error": "method not allowed",
		})
	})
	router.HandleMethodNotAllowed = true

	router.GET("/ping", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"timestamp": time.Now().Format(time.RFC3339Nano),
		})
	})
	router.Run(":8080")
}
