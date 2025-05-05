package auth_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/api/auth"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestEnforcer(t *testing.T) {
	t.Parallel()

	t.Run("superadmin should have access to endpoint", func(t *testing.T) {
		t.Parallel()

		enforcer, err := auth.NewEnforcer()
		require.NoError(t, err, "failed to create enforcer")

		ok, err := enforcer.Enforce("superadmin", "/api/v1/admin", "GET")
		require.NoError(t, err, "failed to enforce policy")
		assert.True(t, ok, "expected policy to be enforced")
	})
}
