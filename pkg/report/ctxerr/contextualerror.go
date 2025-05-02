// ctxerr is a package that provides a very, very cheap contextual error wrapper.
// It is intended to be used in report parsers only, where the error can help to determine where in the parser the error occurred.
// The string should generally be a string literal, and should not be used to pass dynamic data.
package ctxerr

var _ error = (*ContextualError)(nil)

// ContextualError is a very, very cheap contextual error wrapper.
// It contains a context string and an error. The string is usually a string literal, hence not allocated on the heap anywhere.
type ContextualError struct {
	err error
	ctx string
}

func (e *ContextualError) Error() string {
	return e.ctx + ": " + e.err.Error()
}

func (e *ContextualError) Unwrap() error {
	return e.err
}

func (e *ContextualError) Context() string {
	return e.ctx
}

// New creates a new ContextualError with the given error and context.
func New(err error, ctx string) *ContextualError {
	return &ContextualError{
		err: err,
		ctx: ctx,
	}
}
