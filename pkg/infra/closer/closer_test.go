package closer_test

import (
	"errors"
	"testing"

	"github.com/proximyst/cov/pkg/infra/closer"
	"github.com/stretchr/testify/require"
)

func TestClose(t *testing.T) {
	t.Parallel()

	t.Run("does nothing when empty", func(t *testing.T) {
		t.Parallel()

		c := closer.New()
		require.NoError(t, c.Close(), "should not return an error when empty")
	})

	t.Run("ordering is reversed", func(t *testing.T) {
		t.Parallel()

		var called []string
		c := closer.New()
		c.AddFunc(func() { called = append(called, "a") })
		c.AddFunc(func() { called = append(called, "b") })
		c.AddFunc(func() { called = append(called, "c") })

		require.NoError(t, c.Close(), "should not return an error on closing")
		require.Equal(t, []string{"c", "b", "a"}, called, "should call in reverse order")
	})

	t.Run("should return unwrapped error if only one error", func(t *testing.T) {
		t.Parallel()

		errTest := errors.New("test error")
		c := closer.New()
		c.Add(func() error {
			return errTest
		})
		c.AddFunc(func() {})

		err := c.Close()
		require.Same(t, errTest, err, "should return the unwrapped error")
	})

	t.Run("should return all errors if multiple", func(t *testing.T) {
		t.Parallel()

		errTest1 := errors.New("test error 1")
		errTest2 := errors.New("test error 2")
		c := closer.New()
		c.Add(func() error {
			return errTest1
		})
		c.Add(func() error {
			return errTest2
		})

		err := c.Close()
		require.Error(t, err, "should return an error")
		require.ErrorIs(t, err, errTest1)
		require.ErrorIs(t, err, errTest2)
	})
}
