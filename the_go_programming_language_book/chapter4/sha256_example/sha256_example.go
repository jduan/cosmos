package main

import (
	"crypto/sha256"
	"crypto/sha3"
	"flag"
	"fmt"
	"io"
	"os"
)

func main() {
	var algo = flag.String("algo", "sha256", "the algorith to use (sha256, sha384, or sha512)")
	flag.Parse()
	data, err := io.ReadAll(os.Stdin)
	if err != nil {
		fmt.Printf("Error reading input: %s\n", err)
		return
	}

	if *algo == "sha256" {
		c := sha256.Sum256(data)
		fmt.Printf("sha256 of stdin: %x\n", c)
	} else if *algo == "sha384" {
		c := sha3.Sum384(data)
		fmt.Printf("sha384 of stdin: %x\n", c)
	} else if *algo == "sha512" {
		c := sha3.Sum512(data)
		fmt.Printf("sha512 of stdin: %x\n", c)
	} else {
		fmt.Printf("Invalid algo: %s\n", *algo)
		return
	}
}
