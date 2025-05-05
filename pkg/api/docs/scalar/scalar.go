package scalar

import (
	_ "embed"
	"net/http"

	"github.com/gin-gonic/gin"
)

//go:embed index.html
var indexHtml []byte

func Serve(c *gin.Context) {
	c.Data(http.StatusOK, "text/html; charset=utf-8", indexHtml)
}
