package cursor_test

import (
	"io"
	"testing"

	"github.com/proximyst/cov/pkg/report/cursor"
	"github.com/stretchr/testify/require"
)

func TestCursor(t *testing.T) {
	t.Parallel()

	t.Run("read byte", func(t *testing.T) {
		t.Parallel()
		t.Run("with more data", func(t *testing.T) {
			t.Parallel()

			data := []byte("hello, world!")
			c := cursor.Wrap(data)

			b, err := c.ReadByte()
			require.NoError(t, err, "failed to read byte")
			require.Equal(t, byte('h'), b, "expected byte to be 'h'")

			b, err = c.ReadByte()
			require.NoError(t, err, "failed to read byte")
			require.Equal(t, byte('e'), b, "expected byte to be 'e'")
		})

		t.Run("at EOF", func(t *testing.T) {
			t.Parallel()

			c := cursor.Wrap(nil)

			_, err := c.ReadByte()
			require.ErrorIs(t, err, io.EOF, "expected error at EOF")
		})
	})

	t.Run("mark and reset", func(t *testing.T) {
		t.Parallel()

		data := []byte("hello, world!")
		c := cursor.Wrap(data)
		c.Mark()

		b, err := c.ReadByte()
		require.NoError(t, err, "failed to read byte")
		require.Equal(t, byte('h'), b, "expected byte to be 'h'")

		c.Reset()
		b, err = c.ReadByte()
		require.NoError(t, err, "failed to read byte")
		require.Equal(t, byte('h'), b, "expected byte to be 'h' after reset")
	})

	t.Run("peek", func(t *testing.T) {
		t.Parallel()
		t.Run("with more data", func(t *testing.T) {
			t.Parallel()

			data := []byte("hello, world!")
			c := cursor.Wrap(data)
			b, err := c.Peek()
			require.NoError(t, err, "failed to peek byte")
			require.Equal(t, 'h', rune(b), "expected byte to be 'h'")
		})

		t.Run("at EOF", func(t *testing.T) {
			t.Parallel()

			c := cursor.Wrap(nil)
			_, err := c.Peek()
			require.ErrorIs(t, err, io.EOF, "expected error at EOF")
		})
	})

	t.Run("read line", func(t *testing.T) {
		t.Parallel()
		t.Run("with more data", func(t *testing.T) {
			t.Parallel()
			t.Run("with line ending", func(t *testing.T) {
				t.Parallel()

				data := []byte("hello, world!\n")
				c := cursor.Wrap(data)
				line, err := c.ReadLine()
				require.NoError(t, err, "failed to read line")
				require.Equal(t, []byte("hello, world!"), line, "expected line to be 'hello, world!'")
			})

			t.Run("without line ending", func(t *testing.T) {
				t.Parallel()

				data := []byte("hello, world!")
				c := cursor.Wrap(data)
				line, err := c.ReadLine()
				require.NoError(t, err, "failed to read line")
				require.Equal(t, []byte("hello, world!"), line, "expected line to be 'hello, world!'")
			})

			t.Run("with \\r\\n line ending", func(t *testing.T) {
				t.Parallel()

				data := []byte("hello, world!\r\n")
				c := cursor.Wrap(data)
				line, err := c.ReadLine()
				require.NoError(t, err, "failed to read line")
				require.Equal(t, []byte("hello, world!"), line, "expected line to be 'hello, world!'")
			})
		})

		t.Run("at EOF", func(t *testing.T) {
			t.Parallel()

			c := cursor.Wrap(nil)
			_, err := c.ReadLine()
			require.ErrorIs(t, err, io.EOF, "expected error at EOF")
		})
	})
}
