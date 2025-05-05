package ptr

// To returns a pointer to the value passed in.
// This is useful for creating pointers to values in a generic way without temporary variables.
func To[T any](v T) *T {
	return &v
}
