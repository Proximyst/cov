package secret_test

import (
	"encoding/json"
	"testing"

	"github.com/proximyst/cov/pkg/secret"
	"github.com/stretchr/testify/assert"
)

func TestStringIsRedacted(t *testing.T) {
	t.Parallel()
	secretValue := secret.New("hello, world!")

	assert.Equal(t, "secret.Value{value: <redacted>}", secretValue.String())
	assert.Equal(t, "secret.Value{value: <redacted>}", secretValue.GoString())
}

func TestMarshalingReturnsError(t *testing.T) {
	t.Parallel()
	secretValue := secret.New("hello, world!")

	t.Run("binary", func(t *testing.T) {
		t.Parallel()
		_, err := secretValue.MarshalBinary()
		assert.ErrorIs(t, err, secret.ErrMarshalUnsupported)
	})

	t.Run("text", func(t *testing.T) {
		t.Parallel()
		_, err := secretValue.MarshalText()
		assert.ErrorIs(t, err, secret.ErrMarshalUnsupported)
	})

	t.Run("json", func(t *testing.T) {
		t.Parallel()
		_, err := secretValue.MarshalJSON()
		assert.ErrorIs(t, err, secret.ErrMarshalUnsupported)

		_, err = json.Marshal(secretValue)
		assert.ErrorIs(t, err, secret.ErrMarshalUnsupported)
	})
}
