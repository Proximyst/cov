package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/goccy/go-yaml"
	"github.com/urfave/cli/v3"
)

func main() {
	if err := cmd().Run(context.Background(), os.Args); err != nil {
		panic(err)
	}
}

func cmd() *cli.Command {
	return &cli.Command{
		Name:  "yaml2json",
		Usage: "Convert YAML files to JSON format.",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:     "input",
				Aliases:  []string{"i"},
				Usage:    "Input YAML file to convert.",
				Required: true,
			},
			&cli.StringFlag{
				Name:     "output",
				Aliases:  []string{"o"},
				Usage:    "Output JSON file.",
				Required: true,
			},
			&cli.BoolFlag{
				Name:    "pretty",
				Aliases: []string{"p"},
				Usage:   "Pretty print the JSON output.",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			return run(c.String("input"), c.String("output"), c.Bool("pretty"))
		},
	}
}

func run(input, output string, pretty bool) error {
	contents, err := os.ReadFile(input)
	if err != nil {
		return fmt.Errorf("failed to read input file '%s': %w", input, err)
	}

	stat, err := os.Stat(input)
	if err != nil {
		return fmt.Errorf("failed to stat input file '%s': %w", input, err)
	}

	var data map[string]any
	if err := yaml.Unmarshal(contents, &data); err != nil {
		return fmt.Errorf("failed to unmarshal YAML: %w", err)
	}

	marshal := json.Marshal
	if pretty {
		marshal = func(v any) ([]byte, error) {
			return json.MarshalIndent(v, "", "  ")
		}
	}

	jsonData, err := marshal(data)
	if err != nil {
		return fmt.Errorf("failed to marshal JSON: %w", err)
	}

	if err := os.WriteFile(output, jsonData, stat.Mode()); err != nil {
		return fmt.Errorf("failed to write output file '%s': %w", output, err)
	}

	slog.Info("YAML to JSON conversion completed successfully",
		"input", findFileName(input),
		"output", findFileName(output))
	return nil
}

func findFileName(name string) string {
	dir, err := os.Getwd()
	if err != nil {
		return name
	}

	return filepath.Join(dir, name)
}
