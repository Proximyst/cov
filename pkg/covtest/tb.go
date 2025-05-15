package covtest

import "context"

type TestingTB interface {
	Helper()
	Cleanup(func())
	Context() context.Context
	Fatalf(format string, args ...any)
}
