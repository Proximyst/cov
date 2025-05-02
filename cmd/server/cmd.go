package server

import (
	"context"
	"log/slog"

	"github.com/urfave/cli/v3"
)

func New() *cli.Command {
	return &cli.Command{
		Name:  "server",
		Usage: "Start the server.",
		Action: func(ctx context.Context, c *cli.Command) error {
			slog.Info("TODO")
			return nil
		},
	}
}
