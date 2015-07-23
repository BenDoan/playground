package main

import (
	"fmt"
	"path/filepath"
)

func main() {
	path := "this/is/a/path"

	fmt.Printf("Normal path: %v\n\n", path)
	fmt.Printf("Fromslashed path: %v\n\n", filepath.FromSlash(path))
	fmt.Printf("Joined path: %v\n\n", filepath.Join("this", "is", "a", "path"))
	fmt.Printf("Slashed and joined path: %v\n\n", filepath.FromSlash(filepath.Join("this", "is", "a", "path")))
}
