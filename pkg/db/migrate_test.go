package db_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/db"
	"github.com/stretchr/testify/require"
)

func TestEmbeddedMigrationsExist(t *testing.T) {
	// This just ensures that we are embedding migrations correctly.
	// If this test fails, it means we're failing to embed the migrations somewhere.
	t.Parallel()

	src, err := db.EmbeddedMigrationsSource()
	require.NoError(t, err)
	defer src.Close()

	_, err = src.First()
	require.NoError(t, err)
}
