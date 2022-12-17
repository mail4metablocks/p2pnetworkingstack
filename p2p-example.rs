use libp2p::{
    identity,
    multiaddr::{Multiaddr, Protocol},
    peer::PeerId,
    tokio_rustls::rustls::{Certificate, PrivateKey},
    MultiaddrExt,
};
use std::{
    env,
    error::Error,
    io::{self, BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a libp2p host with a random peer ID and listen on a random port
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let transport = libp2p::tcp::TcpConfig::new().nodelay(true);
    let mut host = libp2p::Host::new(transport, local_peer_id, local_key)?;
    let listen_addr = "/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?;
    host.listen_on(listen_addr).await?;

    // Parse the peer ID of the other host from the command-line arguments
    let other_peer_id = env::args()
        .nth(1)
        .expect("Please provide the ID of the other host as an argument")
        .parse::<PeerId>()?;

    // Print the host's peer ID and listen address
    println!("Peer ID: {}", host.peer_id());
    for addr in host.addresses() {
        println!("Listen address: {}", addr);
    }

    // Set up a stream between the two hosts
    let protocol = "/example-protocol/1.0.0".parse::<Protocol>()?;
    let mut stream = host.new_stream(context::Context::background(), &other_peer_id, protocol).await?;

    // Send a message from one host to the other
    writeln!(stream, "Hello, world!")?;

    // Read the response from the other host
    let mut reader = BufReader::new(&mut stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Response: {}", response);

    Ok(())
}
