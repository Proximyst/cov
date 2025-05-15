package covtest

import (
	"context"

	"github.com/alecthomas/kong"
	"github.com/proximyst/cov/pkg/infra/closer"
)

func Run(tb TestingTB, cmd any, args []string, binds ...any) error {
	tb.Helper()
	closer := closer.ForTesting(tb)

	var option []kong.Option
	for _, bind := range binds {
		if o, ok := bind.(kong.Option); ok {
			option = append(option, o)
		} else {
			option = append(option, kong.Bind(bind))
		}
	}
	option = append(option, kong.Bind(closer),
		kong.BindTo(tb.Context(), (*context.Context)(nil)),
		kong.BindTo(tb, (*TestingTB)(nil)))

	parser, err := kong.New(cmd, option...)
	if err != nil {
		tb.Fatalf("failed to create parser: %v", err)
		panic("unreachable")
	}

	c, err := parser.Parse(args)
	if err != nil {
		tb.Fatalf("failed to parse command: %v", err)
		panic("unreachable")
	}

	return c.Run()
}
