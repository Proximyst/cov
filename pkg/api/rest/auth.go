package rest

import (
	"log/slog"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/proximyst/cov/pkg/auth"
	"github.com/proximyst/cov/pkg/db"
	"github.com/proximyst/cov/pkg/db/pgutil"
	"github.com/proximyst/cov/pkg/ptr"
)

func (s *server) Login(c *gin.Context) {
	logger := slog.With("endpoint", "login")

	var body LoginJSONBody
	if err := c.BindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, ErrorResponse{
			Error:       ErrorResponseErrorInvalidBody,
			Description: ptr.To("failed to parse request body"),
		})
		return
	}
	logger = logger.With("username", body.Username, "has-password", body.Password != "")

	if body.Username == "" || body.Password == "" {
		logger.Debug("empty credentials")
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("one or both of username and password are empty"),
		})
		return
	}

	// TODO: implement login logic. return in a cookie.
	usr, err := db.New(s.pool).GetUserWithOptionalPasswordByUsername(c, body.Username)
	if err != nil || usr.Password == nil {
		logger.Debug("user not found or has no password", "error", err)
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("user or password is incorrect"),
		})
		return
	}

	correct, err := auth.VerifyEncoded([]byte(body.Password), *usr.Password)
	if err != nil || !correct {
		logger.Debug("user password does not match", "error", err)
		c.JSON(http.StatusUnauthorized, ErrorResponse{
			Error:       ErrorResponseErrorInvalidCredentials,
			Description: ptr.To("user or password is incorrect"),
		})
		return
	}

	sessionToken, err := auth.GenerateSessionToken()
	if err != nil {
		logger.Error("failed to generate session token", "error", err)
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To("failed to generate session token"),
		})
		return
	}

	tx, err := s.pool.Begin(c)
	if err != nil {
		logger.Error("failed to create tx", "error", err)
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To("failed to begin transaction"),
		})
		return
	}
	defer tx.Rollback(c)
	queries := db.New(tx)

	expiry := auth.SessionExpiry(time.Now())
	if _, err := queries.CreateUserSession(c, db.CreateUserSessionParams{
		ID:           usr.ID,
		SessionToken: sessionToken,
		Expiry:       pgutil.FromTime(expiry),
	}); err != nil {
		logger.Error("failed to create user session", "error", err)
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To("failed to create session"),
		})
		return
	}
	if _, err := queries.CreateAuditLogEvent(c, db.CreateAuditLogEventParams{
		EventType: db.AuditLogEventTypeUserSessionInserted,
		EventData: db.AuditLogEventData{
			UserSessionInserted: &db.AuditLogUserSessionInserted{
				UserID: usr.ID.String(),
				Token:  sessionToken,
				Expiry: expiry,
				Method: "password-login",
			},
		},
	}); err != nil {
		logger.Error("failed to create audit log event", "error", err)
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To("failed to create audit log event"),
		})
		return
	}

	if err := tx.Commit(c); err != nil {
		logger.Error("failed to commit tx", "error", err)
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:       ErrorResponseErrorInternalServerError,
			Description: ptr.To("failed to commit transaction"),
		})
		return
	}

	c.Status(http.StatusNoContent)
	c.SetCookie("session", sessionToken, int(time.Until(expiry).Seconds()), "/", "", false, true)
	logger.Info("user logged in with new session")
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
