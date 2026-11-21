package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func writeToPDF(content string) error {
	cmd := exec.Command("./bin/pdf")
	cmd.Stdin = strings.NewReader(content)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	return cmd.Run()
}

func main() {
	// Command-line arguments
	inputPath := flag.String("n", "", "nasin lon pi ilo sona (path)")
	inputContent := flag.String("o", "", "pana open pi sitelen (input content directly)")

	flag.Parse()

	var content string

	if *inputPath != "" {
		data, err := os.ReadFile(*inputPath)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
			os.Exit(1)
		}
		content = string(data)
	} else {
		content = *inputContent
	}

	if err := writeToPDF(content); err != nil {
		fmt.Fprintf(os.Stderr, "Error writing PDF: %v\n", err)
		os.Exit(1)
	}
}
