package admin

import (
	"github.com/proximyst/cov/cmd/admin/useradd"
	"github.com/proximyst/cov/cmd/admin/userdel"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "admin",
		Usage: "Administration commands that go beyond admin users.",
		Commands: []*cli.Command{
			useradd.New(),
			userdel.New(),
		},
	}
}
