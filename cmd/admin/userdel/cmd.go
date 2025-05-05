package userdel

import (
	"context"

	"github.com/proximyst/cov/pkg/db"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:    "userdel",
		Aliases: []string{"deluser"},
		Usage:   "Delete a user from the database.",
		Flags: []cli.Flag{
			db.FlagConnectionString(),
		},
		MutuallyExclusiveFlags: []cli.MutuallyExclusiveFlags{
			{
				Flags: [][]cli.Flag{
					{
						&cli.StringFlag{
							Name:     "username",
							Usage:    "The username of the user to delete.",
							Category: "User details",
						},
						&cli.StringFlag{
							Name:     "id",
							Usage:    "The ID of the user to delete.",
							Category: "User details",
						},
					},
				},
				Required: true,
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			return run(ctx, c.String("username"), c.String("id"), c.String("database"))
		},
	}
}
