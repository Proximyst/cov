package db

import (
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/proximyst/cov/pkg/infra/env"
	"github.com/urfave/cli/v3"
)

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
