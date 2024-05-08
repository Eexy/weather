package dotenv

import (
	"fmt"
	"os"
	"path"
	"strings"
)

// Parse parses environment variables contains in the '.env' file at the root
func Parse() error {
	data, err := readFile("./.env")
	if err != nil {
		return err
	}

	extractValues(strings.Split(data, "\n"))
	return nil
}

// ParseFiles parses environment variables contains in the specified files
func ParseFiles(files ...string) {
	for _, file := range files {
		data, err := readFile(file)
		if err != nil {
			fmt.Printf("Error reading file %s: %s\n", file, err)
		} else {
			extractValues(strings.Split(data, "\n"))
		}
	}
}

func extractValues(lines []string) {
	for _, line := range lines {
		pair := strings.Split(line, "=")
		if len(pair) == 2 {
			name := strings.ToUpper(strings.TrimSpace(pair[0]))
			value := strings.TrimSpace(pair[1])
			os.Setenv(name, value)
		}
	}
}

func readFile(filename string) (string, error) {
	wd, err := os.Getwd()
	if err != nil {
		return "", err
	}

	data, err := os.ReadFile(path.Join(wd, filename))
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%s", data), nil
}
