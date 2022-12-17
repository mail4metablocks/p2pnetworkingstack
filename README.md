# p2pnetworkingstack


Peer-to-peer (P2P) networking is a type of computer networking in which each computer (peer) in the network acts as both a client and a server. In other words, each computer is able to request and provide resources to and from other computers in the network without the need for a central server.

A P2P networking stack refers to the software and protocols that enable P2P communication between computers. This includes the various layers of the networking stack, such as the application layer, transport layer, network layer, and link layer.

At the application layer, P2P networking stacks typically include protocols for specific types of applications, such as file sharing, messaging, and online gaming. At the transport layer, P2P networking stacks typically use protocols such as TCP or UDP to establish connections and transmit data between computers. At the network layer, P2P networking stacks typically use protocols such as IP to route data between computers. At the link layer, P2P networking stacks typically use protocols such as Ethernet or WiFi to transmit data over physical connections.

P2P networking stacks can be used to create decentralized systems in which there is no central authority controlling the flow of information or resources. This can be useful in situations where a central server is not practical or desirable, such as in the case of file sharing or decentralized social networks.


![image](https://user-images.githubusercontent.com/117555665/208230002-838bd21c-546a-4218-ad63-6b7d0788bbca.png)

### sequence diagram:

Host A initiates a new stream with Host B using the NewStream method.
Host A writes a message to the stream using the Write method.
Host B reads the message from the stream using the Read method.
Host A closes the stream using the Close method.
