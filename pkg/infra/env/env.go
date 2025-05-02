package env

import (
	"os"
	"strings"
)

const Prefix = "COV_"

// Get retrieves the value of the environment variable named by key.
// This calls `os.LookupEnv` and returns the value if it exists, otherwise it returns the fallback value.
//
// This asserts that the `key` has a prefix of `Prefix`.
func Get(key, fallback string) string {
	if !strings.HasPrefix(key, Prefix) {
		key = Prefix + key
	}

	if value, ok := os.LookupEnv(key); ok {
		return value
	}
	return fallback
}

// GetBool retrieves the value of the environment variable named by key.
// See `Get` for details on the key and how it's read.
//
// When the found value is "true", it returns true. Any other value returns false.
// If there is no value found, it returns the fallback value.
func GetBool(key string, fallback bool) bool {
	if value, ok := os.LookupEnv(key); ok {
		return value == "true"
	}
	return fallback
}
