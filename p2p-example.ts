import { PeerId, PeerInfo, createNode } from '@libp2p/js-libp2p';
import { multiaddr } from 'multiaddr';

async function main() {
  // Create a libp2p host with a random peer ID and listen on a random port
  const localNode = await createNode();
  console.log(`Peer ID: ${localNode.peerId.toB58String()}`);
  console.log(`Listen address: ${localNode.multiaddrs[0]}`);

  // Parse the peer ID and listen address of the other host from the command-line arguments
  const otherPeerId = PeerId.createFromB58String(process.argv[2]);
  const otherListenAddr = multiaddr(process.argv[3]);

  // Create a PeerInfo object for the other host
  const otherPeerInfo = new PeerInfo(otherPeerId);
  otherPeerInfo.multiaddrs.add(otherListenAddr);

  // Connect to the other host and set up a stream
  const conn = await localNode.dialProtocol(otherPeerInfo, '/example-protocol');
  const stream = conn.newStream();

  // Send a message from one host to the other
  stream.write('Hello, world!\n');

  // Read the response from the other host
  const response = await new Promise((resolve, reject) => {
    stream.on('data', (chunk: Buffer) => {
      resolve(chunk.toString());
    });
  });
  console.log(`Response: ${response}`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
