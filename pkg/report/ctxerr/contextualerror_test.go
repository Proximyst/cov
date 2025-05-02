package ctxerr_test

import (
	"errors"
	"testing"

	"github.com/proximyst/cov/pkg/report/ctxerr"
	"github.com/stretchr/testify/assert"
)

func TestContextualError(t *testing.T) {
	t.Parallel()

	t.Run("unwrapping returns inner", func(t *testing.T) {
		t.Parallel()

		inner := errors.New("inner error")
		err := ctxerr.New(inner, "context")
		assert.ErrorIs(t, err, inner, "unwrapping should return the inner error")
	})

	t.Run("returns the context as given", func(t *testing.T) {
		t.Parallel()

		inner := errors.New("inner error")
		err := ctxerr.New(inner, "context")
		assert.Equal(t, "context", err.Context(), "context should be the same as given")
	})

	t.Run("prepends context to the error message", func(t *testing.T) {
		t.Parallel()

		inner := errors.New("inner error")
		err := ctxerr.New(inner, "context")
		assert.Equal(t, "context: inner error", err.Error(), "error message should be prepended with the context")
	})
}
