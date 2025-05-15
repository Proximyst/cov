package db

import (
	"context"
	"fmt"

	"github.com/alecthomas/kong"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/proximyst/cov/pkg/infra/closer"
	"github.com/proximyst/cov/pkg/infra/env"
	"github.com/urfave/cli/v3"
)

type Flags struct {
	Database string `help:"Database connection string." default:"host=localhost port=5432 user=cov password=cov dbname=cov sslmode=disable pool_max_conns=10"`
}

func (f *Flags) AfterApply(c *kong.Context, ctx context.Context, closer *closer.C) error {
	pool, err := Connect(ctx, f.Database)
	if err != nil {
		return fmt.Errorf("failed to connect to database: %w", err)
	}
	c.Bind(pool)
	closer.AddFunc(pool.Close)
	return nil
}

func FlagConnectionString() cli.Flag {
	return &cli.StringFlag{
		Name:  "database",
		Usage: "Database connection string.",
		Value: env.Get("DATABASE", "host=localhost port=5432 user=cov password=cov dbname=cov sslmode=disable pool_max_conns=10"),
		Validator: func(value string) error {
			if value == "" {
				return cli.Exit("database connection string cannot be empty", 1)
			}
			_, err := pgxpool.ParseConfig(value)
			if err != nil {
				return cli.Exit("invalid database connection string: "+err.Error(), 1)
			}
			return nil
		},
	}
}
