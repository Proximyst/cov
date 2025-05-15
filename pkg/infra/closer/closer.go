package closer

import (
	"strings"
	"sync"
)

// C keeps track of clean up functions that should be called on application shutdown.
// It is safe to call Add from multiple goroutines.
type C struct {
	mu      *sync.Mutex
	closers []func() error
}

func New() *C {
	return &C{mu: &sync.Mutex{}}
}

func (c *C) Add(closer func() error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.closers = append(c.closers, closer)
}

func (c *C) AddFunc(closer func()) {
	c.Add(func() error {
		closer()
		return nil
	})
}

func (c *C) Close() error {
	c.mu.Lock()
	defer c.mu.Unlock()

	// We close in reverse order to behave similarly to defer.
	var errs []error
	for i := len(c.closers) - 1; i >= 0; i-- {
		if err := c.closers[i](); err != nil {
			errs = append(errs, err)
		}
	}
	if len(errs) == 0 {
		return nil
	} else if len(errs) == 1 {
		return errs[0]
	}
	return errorList(errs)
}

type errorList []error

func (e errorList) Error() string {
	b := &strings.Builder{}
	b.WriteString("multiple errors occurred:\n")
	for i, err := range e {
		if i > 0 {
			b.WriteString("\n")
		}
		b.WriteString("\t")
		b.WriteString(err.Error())
	}
	return b.String()
}

func (e errorList) Unwrap() []error {
	return e
}

type testingTB interface {
	Helper()
	Cleanup(func())
}

func ForTesting(tb testingTB) *C {
	tb.Helper()
	c := New()
	tb.Cleanup(func() {
		_ = c.Close()
	})
	return c
}
