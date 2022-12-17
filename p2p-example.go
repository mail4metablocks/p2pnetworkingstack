package main

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"strings"

	libp2p "github.com/libp2p/go-libp2p"
	peer "github.com/libp2p/go-libp2p-core/peer"
	protocol "github.com/libp2p/go-libp2p-core/protocol"
)

func main() {
	// Create a libp2p host with a random peer ID and listen on a random port
	host, err := libp2p.New(context.Background())
	if err != nil {
		log.Fatal(err)
	}
	defer host.Close()

	// Parse the peer ID of the other host from the command-line arguments
	otherPeerID, err := peer.IDB58Decode(strings.TrimSpace(string(os.Args[1])))
	if err != nil {
		log.Fatal(err)
	}

	// Print the host's peer ID and listen address
	fmt.Printf("Peer ID: %s\n", host.ID())
	fmt.Printf("Listen address: /ip4/%s/tcp/%d\n", host.Addrs()[0].String(), host.Network().ListenAddresses()[0].TCPAddr().Port)

	// Set up a stream between the two hosts
	stream, err := host.NewStream(context.Background(), otherPeerID, protocol.ID("example-protocol"))
	if err != nil {
		log.Fatal(err)
	}
	defer stream.Close()

	// Send a message from one host to the other
	if _, err := fmt.Fprintln(stream, "Hello, world!"); err != nil {
		log.Fatal(err)
	}

	// Read the response from the other host
	response, err := bufio.NewReader(stream).ReadString('\n')
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Response: %s", response)
}
