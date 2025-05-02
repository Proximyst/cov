package secret

import (
	"encoding"
	"encoding/json"
	"fmt"
)

var ErrMarshalUnsupported = fmt.Errorf("secret.Value does not support marshaling")

// Value is a type that wraps any value.
// When marshaled in any way, it will return an error or a generic, redacted string.
type Value[T any] struct {
	value T
}

func New[T any](value T) *Value[T] {
	return &Value[T]{value: value}
}

func (v *Value[T]) Get() T {
	return v.value
}

func (v *Value[T]) String() string {
	return "secret.Value{value: <redacted>}"
}

func (v *Value[T]) GoString() string {
	return "secret.Value{value: <redacted>}"
}

func (v *Value[T]) MarshalJSON() ([]byte, error) {
	return nil, ErrMarshalUnsupported
}

func (v *Value[T]) MarshalBinary() ([]byte, error) {
	return nil, ErrMarshalUnsupported
}

func (v *Value[T]) MarshalText() ([]byte, error) {
	return nil, ErrMarshalUnsupported
}

var (
	_ fmt.Stringer             = (*Value[any])(nil)
	_ fmt.GoStringer           = (*Value[any])(nil)
	_ json.Marshaler           = (*Value[any])(nil)
	_ encoding.BinaryMarshaler = (*Value[any])(nil)
	_ encoding.TextMarshaler   = (*Value[any])(nil)
)
