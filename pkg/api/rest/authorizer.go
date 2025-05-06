package rest

import (
	"errors"
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/proximyst/cov/pkg/auth"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/ptr"
)

type user struct {
	ID         pgtype.UUID
	Username   string
	Roles      []string
	Authorised bool
}

func userFromContext(c *gin.Context) (*user, bool) {
	usr, ok := c.Get("cov_user")
	if !ok {
		return nil, false
	}
	return usr.(*user), true
}

func authorizer(enforcer auth.Enforcer, q db.Querier) gin.HandlerFunc {
	return func(c *gin.Context) {
		logger := slog.With("component", "authorizer-middleware", "endpoint", c.FullPath(), "method", c.Request.Method)
		session, err := c.Cookie("session")
		if (err != nil && errors.Is(err, http.ErrNoCookie)) || session == "" {
			// No session cookie, so the user is anonymous.
			logger.Debug("no session cookie found, user is anonymous")
			c.Next()
			return
		} else if err != nil {
			logger.Warn("failed to read session cookie", "error", err)
			c.AbortWithStatusJSON(http.StatusInternalServerError, ErrorResponse{
				Error:       ErrorResponseErrorInternalServerError,
				Description: ptr.To("cannot read session cookie"),
			})
			return
		}

		usr, err := q.GetUserByToken(c, session)
		if err != nil && !errors.Is(err, pgx.ErrNoRows) {
			logger.Error("failed to read user token", "error", err)
			c.AbortWithStatusJSON(http.StatusInternalServerError, ErrorResponse{
				Error: ErrorResponseErrorInternalServerError,
			})
			return
		}

		if usr == nil {
			// No user exists with this session, so they are anonymous. This might just be an expired session.
			// Clear the cookie and move onwards.
			logger.Debug("no user found for session, clearing cookie")
			c.SetCookie("session", "", -1, "/", "", false, true)
			c.Next()
			return
		}

		roles := make([]auth.Role, len(usr.Roles))
		for i, role := range usr.Roles {
			roles[i] = auth.Role(role)
		}

		ok, err := enforcer.Enforce(roles, c.FullPath(), c.Request.Method)
		if err != nil {
			logger.Error("failed to enforce policy", "error", err)
			c.AbortWithStatusJSON(http.StatusInternalServerError, ErrorResponse{
				Error: ErrorResponseErrorInternalServerError,
			})
			return
		}

		c.Set("cov_user", &user{
			ID:         usr.ID,
			Username:   usr.Username,
			Roles:      usr.Roles,
			Authorised: ok,
		})
	}
}
