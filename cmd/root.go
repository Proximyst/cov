package cmd

import (
	"github.com/alecthomas/kong"
	"github.com/proximyst/cov/cmd/admin"
	"github.com/proximyst/cov/cmd/migrate"
	"github.com/proximyst/cov/cmd/server"
	"github.com/proximyst/cov/pkg/infra/log"
)

type CLI struct {
	Logger log.LogFlags `embed:"" prefix:"log-"`

	Migrate migrate.Command `cmd:"" help:"Run database migrations."`
	Admin   admin.Command   `cmd:"" help:"Admin commands."`
	Server  server.Command  `cmd:"" help:"Run the server."`
}

func (c *CLI) Parse(args []string, options ...kong.Option) (*kong.Context, error) {
	options = append(options, kong.Name("cov"),
		kong.Description("cov is a code coverage service."))

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
