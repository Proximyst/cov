package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestRun(t *testing.T) {
	t.Parallel()

	dir := t.TempDir()
	err := os.WriteFile(filepath.Join(dir, "test.yaml"), []byte("key: value"), 0644)
	require.NoError(t, err, "failed to write test.yaml")

	err = run(filepath.Join(dir, "test.yaml"), filepath.Join(dir, "test.json"), true)
	require.NoError(t, err, "failed to run conversion")

	contents, err := os.ReadFile(filepath.Join(dir, "test.json"))
	require.NoError(t, err, "failed to read test.json")

	var data map[string]any
	err = json.Unmarshal(contents, &data)
	require.NoError(t, err, "failed to unmarshal JSON")

	require.Len(t, data, 1, "expected JSON to have one key")
	require.Equal(t, "value", data["key"], "expected JSON key 'key' to have value 'value'")
}
