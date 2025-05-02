package main

import (
	"context"
	"log/slog"
	"os"

	"github.com/proximyst/cov/cmd"
)

func main() {
	ctx := context.Background()

	if err := cmd.New().Run(ctx, os.Args); err != nil {
		slog.Error("failed to run command", "error", err)
		os.Exit(1)
	}
}
