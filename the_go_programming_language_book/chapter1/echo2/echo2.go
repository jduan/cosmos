package main

import (
	"fmt"
	"os"
)

func main() {
	// s, sep := "", ""
	for idx, arg := range os.Args[1:] {
		fmt.Printf("line %d: %s\n", (idx + 1), arg)
	}
}
