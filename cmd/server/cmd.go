package server

import (
	"context"

	"github.com/proximyst/cov/pkg/rest"
	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "server",
		Usage: "Start the server.",
		Action: func(ctx context.Context, c *cli.Command) error {
			rest.Start()
			return nil
		},
	}
}
