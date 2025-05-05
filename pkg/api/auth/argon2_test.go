package auth_test

import (
	"testing"

	"github.com/matthewhartstonge/argon2"
	"github.com/proximyst/cov/pkg/api/auth"
	"github.com/stretchr/testify/assert"
)

func TestArgon2Recommendations(t *testing.T) {
	t.Parallel()

	argon := auth.Argon()
	assert.Equal(t, argon2.Mode(argon2.ModeArgon2id), argon.Mode, "Argon2 mode should be Argon2id")

	validCfgs := []struct {
		minMemoryCost   uint32
		minTimeCost     uint32
		maxParallellism uint8
	}{
		{47104, 1, 1},
		{19456, 2, 1},
		{12288, 3, 1},
		{9216, 4, 1},
		{7168, 5, 1},
	}
	for _, cfg := range validCfgs {
		if argon.MemoryCost >= cfg.minMemoryCost && argon.TimeCost >= cfg.minTimeCost && argon.Parallelism <= cfg.maxParallellism {
			// This is a valid configuration
			return
		}
	}

	t.Errorf("Argon2 configuration is not valid: MemoryCost=%d, TimeCost=%d, Parallelism=%d", argon.MemoryCost, argon.TimeCost, argon.Parallelism)
}

func TestArgon2VerifyEncoded(t *testing.T) {
	t.Parallel()

	t.Run("with freshly encoded password", func(t *testing.T) {
		t.Parallel()

		password := []byte("password")
		encoded, err := auth.HashEncoded(password)
		assert.NoError(t, err)

		ok, err := auth.VerifyEncoded(password, encoded)
		assert.NoError(t, err)
		assert.True(t, ok)

		// Test with a different password
		ok, err = auth.VerifyEncoded([]byte("wrongpassword"), encoded)
		assert.NoError(t, err)
		assert.False(t, ok)
	})

	t.Run("with valid, real password", func(t *testing.T) {
		t.Parallel()

		encoded := "$argon2id$v=19$m=47104,t=10,p=1$REoWb0MtVKxAI+tQCgGM9g$00O7CzMJ5cmyOyIWmxhqme4WSbxZqFLPp1jZMvsD2wM"
		password := []byte("test")
		ok, err := auth.VerifyEncoded(password, encoded)
		assert.NoError(t, err)
		assert.True(t, ok)

		// Test with a different password
		ok, err = auth.VerifyEncoded([]byte("wrongpassword"), encoded)
		assert.NoError(t, err)
		assert.False(t, ok)
	})
}
