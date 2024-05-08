package dotenv

import (
	"fmt"
	"os"
	"strings"
)

// Parse environment variables contains in the '.env' file at the root
func Parse() error {
	data, err := readEnvFile()
	if err != nil {
		return err
	}

	lines := strings.Split(string(data), "\n")
	for _, line := range lines {
		pair := strings.Split(line, "=")
		if len(pair) == 2 {
			name := strings.ToUpper(strings.TrimSpace(pair[0]))
			value := strings.TrimSpace(pair[1])
			os.Setenv(name, value)
		}
	}

	return nil
}

func readEnvFile() (string, error) {
	data, err := os.ReadFile(".env")
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%s", data), nil
}
