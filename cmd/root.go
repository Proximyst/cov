package cmd

import (
	"github.com/alecthomas/kong"
	"github.com/proximyst/cov/cmd/migrate"
	"github.com/proximyst/cov/pkg/infra/closer"
	"github.com/proximyst/cov/pkg/infra/log"
)

type CLI struct {
	Logger log.LogFlags `embed:"" prefix:"log-"`

	Migrate migrate.Command `cmd:"" help:"Run database migrations."`
}

func (c *CLI) Parse(args []string, closer *closer.C, options ...kong.Option) (*kong.Context, error) {
	options = append(options, kong.Name("cov"),
		kong.Description("cov is a code coverage service."),
		kong.Bind(closer))

	parser, err := kong.New(c, options...)
	if err != nil {
		return nil, err
	}

	ctx, err := parser.Parse(args)
	if err != nil {
		return nil, err
	}

	return ctx, err
}
