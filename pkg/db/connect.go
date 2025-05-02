package db

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5/pgxpool"
)

var _ DBTX = (*pgxpool.Pool)(nil)

// Connect establishes a connection to the PostgreSQL database using the provided connection string.
// It returns a connection pool and an optional error. If the error is not nil, the connection pool is nil and does not need closing.
//
// The connection string must either be a DSN or a URL. The DSN format is:
//
//	host=localhost port=5432 user=postgres password=secret dbname=mydb sslmode=disable pool_max_conns=10
//
// The URL format is:
//
//	postgres://user:password@localhost:5432/mydb?sslmode=disable&pool_max_conns=10
func Connect(ctx context.Context, connString string) (*pgxpool.Pool, error) {
	cfg, err := pgxpool.ParseConfig(connString)
	if err != nil {
		return nil, fmt.Errorf("invalid connection string: %w", err)
	}

	pool, err := pgxpool.NewWithConfig(ctx, cfg)
	if err != nil {
		return nil, fmt.Errorf("failed to create pool: %w", err)
	}

	return pool, nil
}
