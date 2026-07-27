// diff_viewer.go
package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/sergi/go-diff/diffmatchpatch"
)

func colorize(text, color string, enabled bool) string {
	if !enabled {
		return text
	}
	colors := map[string]string{
		"red":   "\033[91m",
		"green": "\033[92m",
		"blue":  "\033[94m",
		"reset": "\033[0m",
	}
	return colors[color] + text + colors["reset"]
}

func readFile(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func main() {
	var context int
	var ignoreSpace bool
	var unified bool
	var color bool
	var output string
	flag.IntVar(&context, "context", 3, "Context lines")
	flag.BoolVar(&ignoreSpace, "ignore-space", false, "Ignore trailing spaces")
	flag.BoolVar(&unified, "unified", true, "Unified format")
	flag.BoolVar(&color, "color", false, "Force color output")
	flag.StringVar(&output, "output", "", "Output file")
	flag.Parse()

	if flag.NArg() < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <file1> <file2> [options]\n", os.Args[0])
		flag.PrintDefaults()
		os.Exit(1)
	}
	file1 := flag.Arg(0)
	file2 := flag.Arg(1)

	lines1, err := readFile(file1)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading %s: %v\n", file1, err)
		os.Exit(1)
	}
	lines2, err := readFile(file2)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading %s: %v\n", file2, err)
		os.Exit(1)
	}

	if ignoreSpace {
		for i := range lines1 {
			lines1[i] = strings.TrimRight(lines1[i], " \t")
		}
		for i := range lines2 {
			lines2[i] = strings.TrimRight(lines2[i], " \t")
		}
	}

	dmp := diffmatchpatch.New()
	diffs := dmp.DiffMain(strings.Join(lines1, "\n"), strings.Join(lines2, "\n"), false)
	// Для унифицированного вывода используем патчи (упрощённо)
	// Здесь просто выводим в стиле diff с + и -
	var result []string
	for _, d := range diffs {
		switch d.Type {
		case diffmatchpatch.DiffDelete:
			result = append(result, colorize("-"+d.Text, "red", color))
		case diffmatchpatch.DiffInsert:
			result = append(result, colorize("+"+d.Text, "green", color))
		default:
			result = append(result, d.Text)
		}
	}
	out := strings.Join(result, "\n")

	if output != "" {
		err := os.WriteFile(output, []byte(out), 0644)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error writing output: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("Diff saved to %s\n", output)
	} else {
		fmt.Println(out)
	}
}
