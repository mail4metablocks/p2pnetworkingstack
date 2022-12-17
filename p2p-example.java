import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CompletableFuture;

import io.ipfs.api.IPFS;
import io.ipfs.api.MerkleNode;
import io.ipfs.api.NamedStreamable;
import io.ipfs.multihash.Multihash;
import net.tomp2p.dht.FutureGet;
import net.tomp2p.dht.PeerBuilderDHT;
import net.tomp2p.dht.PeerDHT;
import net.tomp2p.p2p.PeerBuilder;
import net.tomp2p.peers.Number160;
import net.tomp2p.peers.PeerAddress;

public class Main {
    public static void main(String[] args) throws Exception {
        // Create a libp2p host with a random peer ID and listen on a random port
        PeerDHT peer = new PeerBuilderDHT(new PeerBuilder(Number160.createHash(args[0])).ports(4001).start()).start();

        // Parse the peer ID of the other host from the command-line arguments
        PeerAddress otherPeer = new PeerAddress(Number160.createHash(args[1]));

        // Print the host's peer ID and listen address
        System.out.println("Peer ID: " + peer.peerID());
        System.out.println("Listen address: " + peer.peerAddress());

        // Send a message to the other host
        FutureGet futureGet = peer.peer().get(otherPeer).start();
        futureGet.awaitUninterruptibly();
        String message = (String) futureGet.data().object();
        System.out.println("Message from other host: " + message);

        // Send a response to the other host
        peer.peer().put(otherPeer).data(new Data("Hello, world!")).start().awaitUninterruptibly();
    }
}
