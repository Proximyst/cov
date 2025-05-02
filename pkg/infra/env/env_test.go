package env_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/infra/env"
	"github.com/stretchr/testify/assert"
)

func TestGet(t *testing.T) {
	t.Run("returns value from environment variable", func(t *testing.T) {
		t.Setenv("COV_TEST_ENV_VAR", "test_value")
		value := env.Get("COV_TEST_ENV_VAR", "fallback_value")
		assert.Equal(t, "test_value", value)
	})

	t.Run("returns fallback value if environment variable is not set", func(t *testing.T) {
		value := env.Get("COV_TEST_ENV_VAR", "fallback_value")
		assert.Equal(t, "fallback_value", value)
	})

	t.Run("returns value from environment variable even if empty", func(t *testing.T) {
		t.Setenv("COV_TEST_ENV_VAR", "")
		value := env.Get("COV_TEST_ENV_VAR", "fallback_value")
		assert.Empty(t, value)
	})

	t.Run("adds COV_ prefix if not present", func(t *testing.T) {
		t.Setenv("COV_TEST_ENV_VAR", "test_value")
		value := env.Get("TEST_ENV_VAR", "fallback_value")
		assert.Equal(t, "test_value", value)
	})

	t.Run("does not find COV_-less prefixed variable", func(t *testing.T) {
		t.Setenv("TEST_ENV_VAR", "test_value")
		value := env.Get("TEST_ENV_VAR", "fallback_value")
		assert.Equal(t, "fallback_value", value)

		value = env.Get("COV_TEST_ENV_VAR", "fallback_value")
		assert.Equal(t, "fallback_value", value)
	})
}

func TestGetBool(t *testing.T) {
	t.Run("returns true for 'true' string", func(t *testing.T) {
		t.Setenv("COV_TEST_BOOL", "true")
		value := env.GetBool("COV_TEST_BOOL", false)
		assert.True(t, value)
	})

	t.Run("returns false for 'false' string", func(t *testing.T) {
		t.Setenv("COV_TEST_BOOL", "false")
		value := env.GetBool("COV_TEST_BOOL", true)
		assert.False(t, value)
	})

	t.Run("returns fallback value if environment variable is not set", func(t *testing.T) {
		value := env.GetBool("COV_TEST_BOOL", true)
		assert.True(t, value)
	})
}
