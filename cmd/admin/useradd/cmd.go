package useradd

import (
	"context"
	"fmt"
	"strings"

	"github.com/proximyst/cov/pkg/api/auth"
	"github.com/proximyst/cov/pkg/db"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:    "useradd",
		Aliases: []string{"adduser"},
		Usage:   "Add a new user to the database.",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:     "username",
				Usage:    "The username of the new user.",
				Required: true,
				Category: "User details",
			},
			&cli.StringFlag{
				Name:     "password",
				Usage:    "The password of the new user.",
				Category: "User details",
			},
			&cli.StringFlag{
				Name:     "email",
				Usage:    "The email of the new user.",
				Category: "User details",
				Validator: func(s string) error {
					if s != "" && !strings.Contains(s, "@") {
						return cli.Exit("invalid email address", 1)
					}
					return nil
				},
			},
			&cli.BoolFlag{
				Name:     "verified-email",
				Usage:    "Whether the email is verified.",
				Value:    true,
				Category: "User details",
			},
			&cli.BoolFlag{
				Name:     "primary-email",
				Usage:    "Whether the email is the primary email for the user.",
				Value:    true,
				Category: "User details",
			},
			&cli.StringSliceFlag{
				Name:     "roles",
				Usage:    "The user's roles.",
				Value:    []string{"user"},
				Category: "User details",
				Validator: func(s []string) error {
					for _, role := range s {
						if !auth.Role(role).IsValid() {
							return cli.Exit(fmt.Sprintf("invalid role: %s (enum: %s)", role, allValidRoles()), 1)
						}
					}
					return nil
				},
			},
			db.FlagConnectionString(),
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			return run(ctx,
				c.String("username"), c.String("password"),
				c.String("email"), c.Bool("verified-email"), c.Bool("primary-email"),
				c.StringSlice("roles"),
				c.String("database"))
		},
	}
}

func allValidRoles() string {
	roles := make([]string, len(auth.AllRoles))
	for i, role := range auth.AllRoles {
		roles[i] = role.String()
	}
	return strings.Join(roles, ", ")
}
