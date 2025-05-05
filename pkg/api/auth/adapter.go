package auth

import (
	_ "embed"
	"errors"
	"strings"

	"github.com/casbin/casbin/v2/model"
	"github.com/casbin/casbin/v2/persist"
)

var _ persist.Adapter = embeddedAdapter{}

var errAdapterNotImplemented = errors.New("adapter function is not implemented")

//go:embed policies.csv
var casbinPolicies string

type embeddedAdapter struct{}

func (embeddedAdapter) LoadPolicy(model model.Model) error {
	for line := range strings.Lines(casbinPolicies) {
		line = strings.TrimSpace(line)
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}

		if err := persist.LoadPolicyLine(line, model); err != nil {
			return err
		}
	}

	return nil
}

func (embeddedAdapter) SavePolicy(model model.Model) error {
	return errAdapterNotImplemented
}

func (embeddedAdapter) AddPolicy(sec string, ptype string, rule []string) error {
	return errAdapterNotImplemented
}

func (embeddedAdapter) RemovePolicy(sec string, ptype string, rule []string) error {
	return errAdapterNotImplemented
}

func (embeddedAdapter) RemoveFilteredPolicy(sec string, ptype string, fieldIndex int, fieldValues ...string) error {
	return errAdapterNotImplemented
}
