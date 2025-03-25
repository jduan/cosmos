package main

import (
	"encoding/gob"
	"fmt"
	"net"
)

func server() {
	// get back a listener
	ln, err := net.Listen("tcp", ":9999")
	if err != nil {
		fmt.Println("Failed to start a tcp server on port 9999", err)
		return
	}
	for {
		c, err := ln.Accept()
		if err != nil {
			fmt.Println("Failed to accept a client", err)
		}

		go handleServerConnection(c)
	}
}

func handleServerConnection(c net.Conn) {
	var msg string
	// The encoding/gob package in Go is used for binary serialization—also
	// known as gob encoding. It allows you to encode Go data structures into a
	// compact binary format and then decode them later.
	err := gob.NewDecoder(c).Decode(&msg)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println("Received msg:", msg)
	}
	c.Close()
}

func client() {
	c, err := net.Dial("tcp", "127.0.0.1:9999")
	if err != nil {
		fmt.Println("Failed to connect to server:", err)
	}
	msg := "Hello, world!"
	fmt.Println("Sending msg:", msg)
	// net.Conn implements the io.Reader and io.Writer interfaces
	err = gob.NewEncoder(c).Encode(msg)
	if err != nil {
		fmt.Println("Failed to send msg:", err)
	}
	c.Close()
}
