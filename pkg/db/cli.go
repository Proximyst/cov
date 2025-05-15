package db

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5/pgxpool"
)

type Flags struct {
	Database string `help:"Database connection string." default:"host=localhost port=5432 user=cov password=cov dbname=cov sslmode=disable pool_max_conns=10"`
}

func (f *Flags) Connect(ctx context.Context) (*pgxpool.Pool, error) {
	pool, err := Connect(ctx, f.Database)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to database: %w", err)
	}
	return pool, nil
}
