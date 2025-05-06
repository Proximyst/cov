package auth

import "github.com/matthewhartstonge/argon2"

// HashEncoded hashes the given password using Argon2id and returns the PHC-encoded hash as a string.
func HashEncoded(password []byte) (string, error) {
	argon := Argon()
	hash, err := argon.HashEncoded(password)
	if err != nil {
		return "", err
	}

	return string(hash), nil
}

// VerifyEncoded verifies the given password against the PHC-encoded hash.
// It returns true if the password matches, false otherwise.
func VerifyEncoded(password []byte, encoded string) (bool, error) {
	return argon2.VerifyEncoded(password, []byte(encoded))
}

func Argon() argon2.Config {
	// Reference: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
	//
	// Recommended settings for Argon2id:
	//
	// * m=47104 (46 MiB), t=1, p=1 (Do not use with Argon2i)
	// * m=19456 (19 MiB), t=2, p=1 (Do not use with Argon2i)
	// * m=12288 (12 MiB), t=3, p=1
	// * m=9216 (9 MiB), t=4, p=1
	// * m=7168 (7 MiB), t=5, p=1

	cfg := argon2.DefaultConfig()
	cfg.Mode = argon2.ModeArgon2id
	cfg.MemoryCost = 47104
	cfg.Parallelism = 1
	cfg.TimeCost = 10
	return cfg
}
