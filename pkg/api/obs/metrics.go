package obs

import (
	"bytes"
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/common/expfmt"
	"github.com/proximyst/cov/pkg/ptr"
)

func (s *server) Metrics(c *gin.Context) {
	metrics, done, err := s.metrics.Gather()
	if err != nil {
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To(fmt.Sprintf("failed to gather metrics: %v", err)),
		})
		return
	}
	defer done()

	rendered := bytes.NewBuffer(nil)

	for _, metric := range metrics {
		_, err := expfmt.MetricFamilyToText(rendered, metric)
		if err != nil {
			c.JSON(http.StatusInternalServerError, ErrorResponse{
				Error:       ErrorResponseErrorInternalServerError,
				Description: ptr.To(fmt.Sprintf("failed to write metrics: %v", err)),
			})
			return
		}
	}

	c.Data(http.StatusOK, "text/plain; charset=utf-8", rendered.Bytes())
}
