package rest

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/proximyst/cov/pkg/ptr"
)

func (s *server) Login(c *gin.Context) {
	var body LoginJSONBody
	if err := c.BindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, ErrorResponse{
			Error:       ErrorResponseErrorInvalidBody,
			Description: ptr.To("failed to parse request body"),
		})
		return
	}

	if body.Username == "" || body.Password == "" {
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("one or both of username and password are empty"),
		})
		return
	}

	// TODO: implement login logic. return in a cookie.
	c.Status(http.StatusNoContent)
}

func (s *server) Logout(c *gin.Context, params LogoutParams) {
	cookie, err := c.Cookie("session")
	if err != nil {
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("failed to read session cookie"),
		})
		return
	}

	if cookie == "" {
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("session cookie is empty"),
		})
		return
	}

	// TODO: implement logout logic. invalidate the session.
	c.SetCookie("session", "", -1, "/", "", false, true)
	c.Status(http.StatusNoContent)
}
