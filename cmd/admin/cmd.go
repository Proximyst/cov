package admin

import (
	"github.com/proximyst/cov/cmd/admin/useradd"
	"github.com/proximyst/cov/cmd/admin/userdel"
)

type Command struct {
	UserAdd useradd.Command `cmd:"useradd" alias:"adduser" help:"Add a new user to the database."`
	UserDel userdel.Command `cmd:"userdel" alias:"deluser" help:"Delete a user from the database."`
}
