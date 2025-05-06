package auth

import (
	"crypto/rand"
	"fmt"
	"time"

	_ "embed"

	"github.com/casbin/casbin/v2"
	"github.com/casbin/casbin/v2/model"
	"github.com/google/uuid"
)

//go:embed auth_model.conf
var casbinModel string

func NewEnforcer() (Enforcer, error) {
	model, err := model.NewModelFromString(casbinModel)
	if err != nil {
		return nil, fmt.Errorf("failed to create casbin model: %w", err)
	}

	enforcer, err := casbin.NewEnforcer(model, embeddedAdapter{})
	if err != nil {
		return nil, fmt.Errorf("failed to create casbin enforcer: %w", err)
	}

	return enforcerFunc(func(roles []Role, object, action string) (bool, error) {
		for _, role := range roles {
			if !role.IsValid() {
				continue
			}
			ok, err := enforcer.Enforce(role.String(), object, action)
			if err != nil {
				return false, fmt.Errorf("failed to enforce policy: %w", err)
			}
			if ok {
				return true, nil
			}
		}
		return false, nil
	}), nil
}

type Enforcer interface {
	// Enforce checks if a user with the given roles has the necessary permissions to perform the action on the object.
	// Additional checks should be performed based on the resource. They might have access to the object type, but not this specific resource.
	//
	// It returns true if the permission is granted, false otherwise.
	// If there is an error, say the policies don't make sense, it returns false and the error.
	// The policies are deny-by-default, so if there is no matching policy for the role, object and action, it will return false.
	Enforce(roles []Role, object, action string) (bool, error)
}

type enforcerFunc func(roles []Role, object, action string) (bool, error)

func (f enforcerFunc) Enforce(roles []Role, object, action string) (bool, error) {
	return f(roles, object, action)
}

// User is a user in the database.
type User struct {
	// ID is the user's unique ID in the database.
	ID uuid.UUID
	// Username is the user's name. It is unique, but not static.
	Username string
	// Roles is the user's role names. These are unique and should correspond to the policies.
	Roles []Role
}

// Role is a role in the database. It is a string, but should be one of the predefined roles.
type Role string

const (
	// RoleSuperadmin is the all-powerful role. It can do anything.
	RoleSuperadmin Role = "superadmin"
	// RoleUser is a generic, pretty powerless user.
	RoleUser Role = "user"
	// RoleAnonymous is a user that is not logged in, or has no permissions within the organisation. It has no permissions.
	RoleAnonymous Role = "anonymous"
)

var AllRoles = []Role{
	RoleSuperadmin,
	RoleUser,
	RoleAnonymous,
}

func (r Role) String() string {
	return string(r)
}

func (r Role) IsValid() bool {
	switch r {
	case RoleSuperadmin, RoleUser, RoleAnonymous:
		return true
	default:
		return false
	}
}

// SessionPrefix is a random string that can be used to identify the origin of any session token.
// This is useful for e.g. Trufflehog or GitHub secret scanning.
const SessionPrefix = "cov_3361aee70c"

// GenerateSessionToken generates a random session token. It is a 48 character long string with a prefix of 14 characters.
// This uses a cryptographically secure random number generator to generate the token.
func GenerateSessionToken() (string, error) {
	const alphabet = "abcdefghijklmnopqrstuvwxyz0123456789"
	const length = 34
	token := make([]byte, length+len(SessionPrefix))
	copy(token, SessionPrefix)
	// Read always fills the buffer entirely and never returns an error.
	_, _ = rand.Read(token[len(SessionPrefix):])
	for i := len(SessionPrefix); i < len(token); i++ {
		token[i] = alphabet[int(token[i])%len(alphabet)]
	}
	return string(token), nil
}

const SessionLifetime = time.Hour * 23

func SessionExpiry(now time.Time) time.Time {
	return now.Add(SessionLifetime)
}
