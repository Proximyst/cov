package auth

import (
	"fmt"

	_ "embed"

	"github.com/casbin/casbin/v2"
	"github.com/casbin/casbin/v2/model"
	"github.com/google/uuid"
)

//go:embed auth_model.conf
var casbinModel string

func NewEnforcer() (*casbin.Enforcer, error) {
	model, err := model.NewModelFromString(casbinModel)
	if err != nil {
		return nil, fmt.Errorf("failed to create casbin model: %w", err)
	}

	enforcer, err := casbin.NewEnforcer(model, embeddedAdapter{})
	if err != nil {
		return nil, fmt.Errorf("failed to create casbin enforcer: %w", err)
	}

	return enforcer, nil
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
