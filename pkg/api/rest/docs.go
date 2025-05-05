package rest

import (
	"github.com/gin-gonic/gin"
	"github.com/proximyst/cov/pkg/api/docs/redoc"
	"github.com/proximyst/cov/pkg/api/docs/scalar"
)

func (s *server) Redoc(c *gin.Context) {
	redoc.Serve(c)
}

func (s *server) Scalar(c *gin.Context) {
	scalar.Serve(c)
}
