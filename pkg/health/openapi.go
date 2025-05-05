package health

import (
	_ "embed"
	"net/http"

	"github.com/gin-gonic/gin"
)

//go:embed openapi.json
var openapiJson []byte

//go:embed openapi.yaml
var openapiYaml []byte

func (s *server) OpenapiJson(c *gin.Context) {
	c.Data(http.StatusOK, "application/json; charset=utf-8", openapiJson)
}

func (s *server) OpenapiYaml(c *gin.Context) {
	c.Data(http.StatusOK, "application/yaml; charset=utf-8", openapiYaml)
}
