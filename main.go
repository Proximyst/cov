package main

import (
	"context"
	"log/slog"
	"os"

	"github.com/alecthomas/kong"
	"github.com/proximyst/cov/cmd"
	"github.com/proximyst/cov/pkg/infra/binds"
	"github.com/proximyst/cov/pkg/infra/closer"
)

func main() {
	os.Exit(run())
}

func run() int {
	ctx := context.Background()

	closer := closer.New()
	defer func() {
		if err := closer.Close(); err != nil {
			slog.Error("failed to clean up after app", "error", err)
		}
	}()

	// Strip out the program name from args.
	args := os.Args[1:]

	options := []kong.Option{binds.Context(ctx)}
	options = append(options, binds.Metrics(nil)...)

	cli := &cmd.CLI{}
	c, err := cli.Parse(args, closer, options...)
	if err != nil {
		slog.Error("failed to parse command", "error", err)
		return 1
	}
	if err := c.Run(); err != nil {
		slog.Error("failed to run command", "error", err)
		return 1
	}
	return 0
}
