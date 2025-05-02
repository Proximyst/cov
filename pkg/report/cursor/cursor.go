package cursor

import "io"

func Wrap(data []byte) *C {
	return &C{
		data: data,
		pos:  0,
		mark: -1,
	}
}

type C struct {
	data []byte
	pos  int
	mark int
}

// Mark sets the current position of the cursor as a mark.
func (c *C) Mark() {
	c.mark = c.pos
}

// Reset resets the cursor to the last marked position.
// If no mark has been set, this does nothing.
// When reset, the mark is cleared.
func (c *C) Reset() {
	if c.mark == -1 {
		return
	}
	c.pos = c.mark
	c.mark = -1
}

func (c *C) Peek() (byte, error) {
	if !c.canRead(1) {
		return 0, io.EOF
	}
	return c.data[c.pos+1], nil
}

// ReadByte reads a single byte from the cursor and advances the cursor position by 1.
// If there is no more data to read, it returns io.EOF.
// When this reads the final byte, it still returns a nil error.
func (c *C) ReadByte() (byte, error) {
	if !c.canRead(1) {
		return 0, io.EOF
	}
	b := c.data[c.pos]
	c.pos++
	return b, nil
}

// ReadLine reads a single line from the cursor and advances the cursor position by the length of the line.
// It returns the line as a byte slice, excluding the line ending.
// If there is no more data to read, it returns io.EOF.
// If no line ending is eventually found, the line is returned. This is likely EOF.
func (c *C) ReadLine() ([]byte, error) {
	if !c.canRead(1) {
		return nil, io.EOF
	}

	start := c.pos
	for c.canRead(1) && c.data[c.pos] != '\n' {
		c.pos++
	}

	line := c.data[start:c.pos]
	if c.canRead(1) {
		c.pos++
	}

	// We might be dealing with a CRLF line ending...
	if len(line) != 0 && line[len(line)-1] == '\r' {
		line = line[:len(line)-1]
	}
	return line, nil
}

// ReadTill reads bytes from the cursor until the specified separator byte is found.
// It returns the bytes read, excluding the separator.
// When returning, it also advances the cursor position by 1 to skip the separator in future calls.
// If there is no more data to read, it returns io.EOF.
func (c *C) ReadTill(separator byte) ([]byte, error) {
	if !c.canRead(1) {
		return nil, io.EOF
	}

	start := c.pos
	for {
		if !c.canRead(1) {
			return nil, io.ErrUnexpectedEOF
		}
		if c.data[c.pos] == separator {
			break
		}
		c.pos++
	}

	line := c.data[start:c.pos]
	c.pos++
	return line, nil
}

func (c *C) canRead(n int) bool {
	return c.pos+n <= len(c.data)
}
