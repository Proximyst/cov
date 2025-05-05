package ptr_test

import (
	"testing"

	"github.com/proximyst/cov/pkg/ptr"
	"github.com/stretchr/testify/require"
)

func TestPointerTo(t *testing.T) {
	t.Parallel()

	s := ptr.To("hello, world!")
	require.NotNil(t, s, "expected pointer to be non-nil")
	require.Equal(t, "hello, world!", *s, "expected pointer to point to the same value")
}
