package covtest

import (
	"fmt"

	"github.com/alecthomas/kong"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/proximyst/cov/pkg/infra/binds"
)

type runOptions struct {
	options []kong.Option
	metrics *prometheus.Registry
}

type option func(o *runOptions)

func WithMetrics(registry *prometheus.Registry) option {
	return func(o *runOptions) {
		o.metrics = registry
	}
}

func WithBinds(bind ...any) option {
	return func(o *runOptions) {
		o.options = append(o.options, kong.Bind(bind...))
	}
}

func WithOptions(options ...kong.Option) option {
	return func(o *runOptions) {
		o.options = append(o.options, options...)
	}
}

func Run(tb TestingTB, cmd any, args []string, opts ...option) error {
	tb.Helper()
	var o runOptions
	for _, opt := range opts {
		opt(&o)
	}
	o.options = append(o.options,
		binds.Context(tb.Context()),
		kong.BindTo(tb, (*TestingTB)(nil)))
	o.options = append(o.options, binds.Metrics(o.metrics)...)

	exitCode := 0
	o.options = append(o.options, kong.Exit(func(code int) {
		exitCode = code
	}))

	parser, err := kong.New(cmd, o.options...)
	if err != nil {
		tb.Fatalf("failed to create parser: %v", err)
		panic("unreachable")
	}

	c, err := parser.Parse(args)
	if err != nil {
		tb.Fatalf("failed to parse command: %v", err)
		panic("unreachable")
	}

	if err := c.Run(); err != nil {
		return err
	}

	if exitCode != 0 {
		return ExitCode(exitCode)
	}

	return nil
}

type ExitCode int

func (e ExitCode) Error() string {
	return fmt.Sprintf("exit code %d", e)
}
