package server

import (
	"context"

	"github.com/gin-gonic/gin"
	"github.com/proximyst/cov/pkg/db"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "server",
		Usage: "Start the server.",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:  "health-address",
				Usage: "The address to bind the health check server to.",
				Value: ":8081",
			},
			&cli.StringFlag{
				Name:  "rest-address",
				Usage: "The address to bind the REST server to.",
				Value: ":8080",
			},
			db.FlagConnectionString(),
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			// The Gin mode should only be release mode when running from the command. Non-e2e tests should use test mode.
			// As such, this isn't set in the run function, but here.
			gin.SetMode(gin.ReleaseMode)

			return run(ctx, c.String("health-address"), c.String("rest-address"), c.String("database"))
		},
	}
}
