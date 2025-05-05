package rest

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/proximyst/cov/pkg/ptr"
	"github.com/proximyst/cov/pkg/report"
)

func (s *server) ParseReport(c *gin.Context) {
	body, err := c.GetRawData()
	if err != nil {
		c.JSON(http.StatusBadRequest, ErrorResponse{
			Error:       ErrorResponseErrorReportInvalid,
			Description: ptr.To("failed to read request body"),
		})
		return
	}

	report, err := report.Parse(body)
	if err != nil {
		c.JSON(http.StatusBadRequest, ErrorResponse{
			Error:       ErrorResponseErrorReportInvalid,
			Description: ptr.To(err.Error()),
		})
		return
	}

	c.JSON(http.StatusOK, report)
}
