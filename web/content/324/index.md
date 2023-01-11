+++
title = "Version 2 P2P Encrypted Transport Protocol"
date = 2019-03-08
weight = 324
in_search_index = true

[taxonomies]
authors = ["Dhruv Mehta", "Tim Ruffing", "Jonas Schnelli", "Pieter Wuille"]
status = ["Draft"]

[extra]
bip = 324
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0324.mediawiki"
+++

``` 
  BIP: 324
  Layer: Peer Services
  Title: Version 2 P2P Encrypted Transport Protocol
  Author: Dhruv Mehta <dhruv@bip324.com>
          Tim Ruffing <crypto@timruffing.de>
          Jonas Schnelli <dev@jonasschnelli.ch>
          Pieter Wuille <bitcoin-dev@wuille.net>
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0324
  Status: Draft
  Type: Standards Track
  Created: 2019-03-08
  License: BSD-3-Clause
  Replaces: 151
```

## Introduction

### Abstract

This document proposes a new Bitcoin P2P transport protocol, which
features opportunistic encryption, a mild bandwidth reduction, and the
ability to negotiate upgrades before exchanging application messages.

### Copyright

This document is licensed under the 3-clause BSD license.

### Motivation

Bitcoin is a permissionless network whose purpose is to reach consensus
over public data. Since all data relayed in the Bitcoin P2P network is
inherently public, and the protocol lacks a notion of cryptographic
identities, peers talk to each other over unencrypted and
unauthenticated connections. Nevertheless, this plaintext nature of the
current P2P protocol (referred to as v1 in this document) has severe
drawbacks in the presence of attackers:

  - While the relayed data itself is public in nature, the associated
    metadata may reveal private information and hamper privacy of users.
    For example, a global passive attacker eavesdropping on all Bitcoin
    P2P connections can trivially identify the source and timing of a
    transaction.
  - Since connections are unauthenticated, they can be tampered with at
    a low cost and often even with a low risk of detection. For example,
    an attacker can alter specific bytes of a connection (such as node
    flags) on-the-fly without the need to keep any state.
  - The protocol is self-revealing. For example, deep packet inspection
    can identify a P2P connection trivially because connections start
    with a fixed sequence of magic bytes. The ability to detect
    connections enables censorship and facilitates the aforementioned
    attacks as well as other attacks which require the attacker to
    control the connections of victims, e.g., eclipse attacks targeted
    at miners.

This proposal for a new P2P protocol version (v2) aims to improve upon
this by raising the costs for performing these attacks substantially,
primarily through the use of unauthenticated, opportunistic transport
encryption. In addition, the bytestream on the wire is made pseudorandom
(i.e., indistinguishable from uniformly random bytes) to a passive
eavesdropper.

  - Encryption, even when it is unauthenticated and only used when both
    endpoints support v2, impedes eavesdropping by forcing the attacker
    to become active: either by performing a persistent
    man-in-the-middle (MitM) attack, by downgrading connections to v1,
    or by spinning up their own nodes and getting honest nodes to make
    connections to them. Active attacks at scale are more resource
    intensive in general, but in case of manual, deliberate connections
    (as opposed to automatic, random ones) they are also in principle
    detectable: even very basic checks, e.g., operators manually
    comparing protocol versions and session IDs (as supported by the
    proposed protocol), will expose the attacker.
  - Tampering, while already an inherently active attack, is costlier if
    the attacker is forced to maintain the state necessary for a full
    MitM interception.
  - A pseudorandom bytestream excludes identification techniques based
    on pattern matching, and makes it easier to shape the bytestream in
    order to mimic other protocols used on the Internet. This raises the
    cost of a connection censoring firewall, forcing them to either
    resort to a full MitM attack, or operate on a more obvious allowlist
    basis, rather than a blocklist basis.

*' Why encrypt without authentication?*'

As we have argued above, unauthenticated
encryption<ref name="what_does_auth_mean">**What does *authentication*
mean in this context?** Unfortunately, the term authentication in the
context of secure channel protocols is ambiguous. It can refer to:

  - The encryption scheme guaranteeing that a message obtained via
    successful decryption was encrypted by someone having access to the
    (symmetric) encryption key, and not modified after encryption by a
    third party. The proposal in this document achieves that property
    through the use of an AEAD.
  - The communication protocol establishing that the communication
    partner's identity matches who we expect them to be, through some
    public key mechanism. The proposal in this document does **not**
    include such a mechanism.</ref> provides strictly better security
    than no encryption. Thus all connections should use encryption, even
    if they are unauthenticated.

When it comes to authentication, the situation is not as clear as for
encryption. Due to Bitcoin's permissionless nature, authentication will
always be restricted to specific scenarios (e.g., connections between
peers belonging to the same operator), and whether some form of
(possibly partially anonymous) authentication is desired depends on the
specific requirements of the involved peers. As a consequence, we
believe that authentication should be addressed separately (if desired),
and this proposal aims to provide a solid technical basis for future
protocol upgrades, including the addition of optional authentication
(see [Private authentication
protocols](https://github.com/sipa/writeups/tree/main/private-authentication-protocols)).

''' Why have a pseudorandom bytestream when traffic analysis is still
possible? '''

Traffic analysis, e.g., observing packet lengths and timing, as well as
active attacks can still reveal that the Bitcoin v2 P2P protocol is in
use. Nevertheless, a pseudorandom bytestream raises the cost of
fingerprinting the protocol substantially, and may force some
intermediaries to attack any protocol they cannot identify, causing
collateral cost.

A pseudorandom bytestream is not self-identifying. Moreover, it is
unopinionated and thus a canonical choice for similar protocols. As a
result, Bitcoin P2P traffic will be indistinguishable from traffic of
other protocols which make the same choice (e.g.,
[obfs4](https://gitlab.com/yawning/obfs4) and a recently proposed [cTLS
extension](https://datatracker.ietf.org/doc/draft-cpbs-pseudorandom-ctls/)).
Moreover, traffic shapers and protocol wrappers (for example, making the
traffic look like HTTPS or SSH) can further mitigate traffic analysis
and active attacks but are out of scope for this proposal.

''' Why not use a secure tunnel protocol? '''

Our goal includes making opportunistic encryption ubiquitously
available, as that provides the best defense against large-scale
attacks. That implies protecting both the manual, deliberate connections
node operators instruct their software to make, as well as the the
automatic connections Bitcoin nodes make with each other based on IP
addresses obtained via gossip. While encryption per se is already
possible with proxy networks or VPN networks, these are not desirable or
applicable for automatic connections at scale:

  - Proxy networks like Tor or I2P introduce a separate address space,
    independent from network topology, with a very low cost per address
    making eclipse attacks cheaper. In comparison, clearnet IPv4 and
    IPv6 networks make obtaining multiple network identities in
    distinct, well-known network partitions carry a non-trivial cost.
    Thus, it is not desirable to have a substantial portion of nodes be
    exclusively connected this way, as this would significantly reduce
    Eclipse attack costs.\[1\] Additionally, Tor connections come with
    significant bandwidth and latency costs that may not be desirable
    for all network users.
  - VPN networks like WireGuard or OpenVPN inherently define a private
    network, which requires manual configuration and therefore is not a
    realistic avenue for automatic connections.

Thus, to achieve our goal, we need a solution that has minimal costs,
works without configuration, and is always enabled – on top of any
network layer rather than be part of the network layer.

''' Why not use a general-purpose transport encryption protocol? '''

While it would be possible to rely on an off-the-shelf transport
encryption protocol such as TLS or Noise, the specific requirements of
the Bitcoin P2P network laid out above make these protocols an
unsuitable choice.

The primary requirement which existing protocols fail to meet is a
sufficiently modular treatment of encryption and authentication. As we
argue above, whether and which form of authentication is desired in the
Bitcoin P2P network will depend on the specific requirements of the
involved peers (resulting in a mix of authenticated and unauthenticated
connections), and thus the question of authentication should be
decoupled from encryption. However, native support for a handful of
standard authentication scenarios (e.g., using digital signatures and
certificates) is at core of the design of existing general-purpose
transport encryption protocols. This focus on authentication would not
provide clear benefits for the Bitcoin P2P network but would come with a
large amount of additional complexity.

In contrast, our proposal instead aims for simple modular design that
makes it possible to address authentication separately. Our proposal
provides a foundation for authentication by exporting a *session ID*
that uniquely identifies the encrypted channel. After an encrypted
channel has been established, the two endpoints are able to use any
authentication protocol to confirm that they have the same session ID.
(This is sometimes called *channel binding* because the session ID binds
the encrypted channel to the authentication protocol.) Since in our
proposal, any authentication needs to run after an encrypted connection
has been established, the price we pay for this modularity is a possibly
higher number of roundtrips as opposed to other protocols that perform
authentication alongside with the Diffie-Hellman key exchange.\[2\]
However, the resulting increase in connection establishment latency is a
not a concern for Bitcoin's long-lived connections, [which typically
live for hours or even weeks](https://www.dsn.kastel.kit.edu/bitcoin/).

Besides this fundamentally different treatment of authentication,
further technical issues arise when applying TLS or Noise to our desired
use case:

  - Neither offers a pseudorandom bytestream.
  - Neither offers native support for elliptic curve cryptography on the
    curve secp256k1 as otherwise used in Bitcoin. While using secp256k1
    is not strictly necessary, it is the obvious choice is for any new
    asymmetric cryptography in Bitcoin because it minimizes the
    cryptographic hardness assumptions as well as the dependencies that
    Bitcoin software will need.
  - Neither offers shapability of the bytestream.
  - Both provide a stream-based interface to the application layer
    whereas Bitcoin requires a packet-based interface, resulting in the
    need for an additional thin layer to perform packet serialization
    and deserialization.

While existing protocols could be amended to address all of the
aforementioned issues, this would negate the benefits of using them as
off-the-shelf solution, e.g., the possibility to re-use existing
implementations and security analyses.

## Goals

This proposal aims to achieve the following properties:

  - Confidentiality against passive attacks: A passive attacker having
    recorded a v2 P2P bytestream (without timing and fragmentation
    information) must not be able to determine the plaintext being
    exchanged by the nodes.
  - Observability of active attacks: A session ID identifying the
    encrypted channel uniquely is derived deterministically from a
    Diffie-Hellman negotiation. An active man-in-the-middle attacker is
    forced to incur a risk of being detected as peer operators can
    compare session IDs manually, or using optional authentication
    methods possibly introduced in future protocol versions.
  - Pseudorandom bytestream: A passive attacker having recorded a v2 P2P
    bytestream (without timing information and fragmentation
    information) must not be able to distinguish it from a uniformly
    random bytestream.
  - Shapable bytestream: It should be possible to shape the bytestream
    to increase resistance to traffic analysis (for example, to conceal
    block propagation), or censorship avoidance.\[3\]
  - Forward secrecy: An eavesdropping attacker who compromises a peer's
    sessions secrets should not be able to decrypt past session traffic,
    except for the latest few packets.
  - Upgradability: The proposal provides an upgrade path using transport
    versioning which can be used to add features like authentication,
    PQC handshake upgrade, etc. in the future.
  - Compatibility: v2 clients will allow inbound v1 connections to
    minimize risk of network partitions.
  - Low overhead: the introduction of a new P2P transport protocol
    should not substantially increase computational cost or bandwidth
    for nodes that implement it, compared to the current protocol.

## Specification

The specification consists of three parts:

  - The **Transport layer** concerns how to set up an encrypted
    connection between two nodes, capable of transporting
    application-level messages between them.
  - The **Application layer** concerns how to encode Bitcoin P2P
    messages and commands for transport by the Transport Layer.
  - The **Signaling** concerns how v2 nodes advertise their support for
    the v2 protocol to potential peers.

### Transport layer specification

In this section we define the encryption protocol for messages between
peers.

#### Overview and design

We first give an informal overview of the entire protocol flow and
packet encryption.

**Protocol flow overview**

Given a newly-established connection (typically TCP/IP) between two v2
P2P nodes, there are 3 phases the connection goes through. The first
starts immediately, i.e. there are no v1 messages or any other bytes
exchanged on the link beforehand. The two parties are called the
**initiator** (who established the connection) and the **responder**
(who accepted the connection).

1.  The **Key exchange phase**, where nodes exchange data to establish
    shared secrets.
      - The initiator:
          - Generates a random ephemeral secp256k1 private key and sends
            a corresponding 64-byte ElligatorSwift\[4\]\[5\]-encoded
            public key to the responder.
          - May send up to 4095\[6\] bytes of arbitrary data after their
            public key, called **garbage**, providing a form of
            shapability and avoiding a recognizable pattern of exactly
            64 bytes.\[7\]
      - The responder:
          - Waits until one byte is received which does not match the 12
            bytes consisting of the network magic followed by
            "version\\x00". If the first 12 bytes do match, the
            connection is treated as using the v1 protocol
            instead.\[8\]\[9\]
          - Similarly generates a random ephemeral private key and sends
            a corresponding 64-byte ElligatorSwift-encoded public key to
            the initiator.
          - Similarly may send up to 4095 bytes of garbage data after
            their public key.
      - Both parties:
          - Receive (the remainder of) the full 64-byte public key from
            the other side.
          - Use X-only\[10\] ECDH to compute a shared secret from their
            private key and the exchanged public keys\[11\], and
            deterministically derive from the secret 4 **encryption
            keys** (two in each direction: one for packet lengths, one
            for content encryption), a **session id**, and two 16-byte
            **garbage terminators**\[12\]<ref>**What does a garbage
            terminator in the wild look like?**
            <div>
            ![A garbage terminator model TX-v2 in the wild... sent by
            the responder](bip-0324/garbage_terminator.png
            "A garbage terminator model TX-v2 in the wild... sent by the responder")
            </div>

</ref> (one in each direction) using HKDF-SHA256.

\#\*\* Send their 16-byte garbage terminator\[13\] followed by a
**garbage authentication packet**\[14\], an **encrypted packet** (see
further) with arbitrary **contents**, and **associated data** equal to
the garbage.

\#\*\* Receive up to 4111 bytes, stopping when encountering the garbage
terminator.

\#\*\* Receive an encrypted packet, verify that it decrypts correctly
with associated data set to the garbage received, and then ignore its
contents.

\#\* At this point, both parties have the same keys, and all further
communication proceeds in the form of encrypted packets. Packets have an
**ignore bit**, which makes them **decoy packets** if set. Decoy packets
are to be ignored by the receiver apart from verifying they decrypt
correctly. Either peer may send such decoy packets at any point after
this. These form the primary shapability mechanism in the protocol. How
and when to use them is out of scope for this document.

1.  The **Version negotiation phase**, where parties negotiate what
    transport version they will use, as well as data defined by that
    version.\[15\]
      - The responder:
          - Sends a **version packet** with empty content, to indicate
            support for the v2 P2P protocol proposed by this document.
            Any other value for content is reserved for future versions.
      - The initiator:
          - Receives a packet, ignores its contents. The idea is that
            features added by future versions get negotiated based on
            what is supported by both parties. Since there is just one
            version so far, the contents here can simply be ignored. But
            in the future, receiving a non-empty contents here may
            trigger other behavior; we defer specifying the encoding for
            such version content until there is a need for it.\[16\]
          - Sends a **version packet** with empty content as well, to
            indicate support for the v2 P2P protocol.
      - The responder:
          - Receives a packet, ignores its contents.
2.  The **Application phase**, where the packets exchanged have contents
    to be interpreted as application data.
      - Whenever either peer has a message to send, it sends a packet
        with that application message as **contents**.

In order to provide a means of avoiding the recognizable pattern of
first messages being at least 64 bytes, a future backwards-compatible
upgrade to this protocol may allow both peers to send their public key +
garbage + garbage terminator in multiple rounds, slicing those bytes up
into messages arbitrarily, as long as progress is
guaranteed.<ref name="handshake_progress">**How can progress be
guaranteed in a backwards-compatible way?** In order to guarantee
progress, it must be ensured that no deadlock occurs, i.e., no state is
reached in which each party waits for the other party indefinitely. For
example, any upgrade that adheres to the following conditions will
guarantee progress:

  - The initiator must start by sending at least as many bytes as
    necessary to mismatch the magic/version 12 bytes prefix.
  - The responder must start sending after having received at least one
    byte that mismatches that 12-byte prefix.
  - As soon as either party has received the other peer's garbage
    terminator, or has received 4095 bytes of garbage, they must send
    their own garbage terminator. (When either of these conditions is
    met, the other party has nothing to respond with anymore that would
    be needed to guarantee progress otherwise.)
  - Whenever either party receives any nonzero number of bytes, while
    not having sent their garbage terminator completely yet, they must
    send at least one byte in response without waiting for more bytes.
  - After either party has sent their garbage terminator, they must also
    send the garbage authentication packet without waiting for more
    bytes, and transition to the version negotiation phase.

Since the protocol as specified here adheres to these conditions, any
upgrade which also adheres to these conditions will be
backwards-compatible.</ref>

Note that the version negotiation phase does not need to wait for the
key exchange phase to complete; version packets can be sent immediately
after sending the garbage authentication packet. So the first two phases
together, jointly called **the handshake**, comprise just 1.5
roundtrips:

  - the initiator sends public key + garbage
  - the responder sends public key + garbage + garbage terminator +
    garbage authentication packet + version packet
  - the initiator sends garbage terminator + garbage authentication
    packet + version packet

**Packet encryption overview**

All data on the wire after the garbage terminators takes the form of
encrypted packets. Every packet encodes an encrypted variable-length
byte array, called the **contents**, as well as an **ignore bit** as
mentioned before. The total size of a packet is 20 bytes plus the length
of its contents.

Each packet consists of:

  - A 3-byte encrypted **length** field, encoding the length of the
    **contents** (between *0* and *2<sup>24</sup>-1*\[17\], inclusive).
  - An authenticated encryption of the **plaintext**, which consists of:
      - A 1-byte **header** which consists of transport layer protocol
        flags. Currently only the highest bit is defined as the **ignore
        bit**. The other bits are ignored, but this may change in future
        versions\[18\].
      - The variable-length **contents**.

The encryption of the plaintext uses
**[ChaCha20Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305)**\[19\],
an [authenticated encryption with associated
data](https://en.wikipedia.org/wiki/Authenticated_encryption) (AEAD)
cipher specified in
[RFC 8439](https://datatracker.ietf.org/doc/html/rfc8439). Every
packet's plaintext is treated as a separate AEAD message, with a
different nonce for each.

The length must be dealt with specially, as it is needed to determine
packet boundaries before the whole packet is received and authenticated.
As we want a stream that is pseudorandom to a passive attacker, it still
needs encryption. We use unauthenticated\[20\] **ChaCha20** encryption
for this, with an independent key. Note that the plaintext length is
still implicitly authenticated by the encryption of the plaintext, but
this can only be verified after receiving the whole packet. This design
is inspired by that of the ChaCha20Poly1305 cipher suite in
[OpenSSH](http://bxr.su/OpenBSD/usr.bin/ssh/PROTOCOL.chacha20poly1305).<ref name="openssl_changes">**How
does packet encryption differ from the OpenSSH design?** The differences
are:

  - The length field is only 3 bytes instead of 4, as that is sufficient
    for our purposes.
  - Length encryption keeps drawing pseudorandom bytes from the same
    ChaCha20 cipher for multiple packets, rather than incrementing the
    nonce for every packet.
  - The Poly1305 authentication tag only covers the encrypted plaintext,
    and not the encrypted length field. This means that plaintext
    encryption uses the standard ChaCha20Poly1305 construction without
    any modifications, maximizing applicability of analysis and review
    of that cipher. The length encryption can be seen as a separate
    layer, using a separate key, and thus cannot affect any of the
    confidentiality or integrity guarantees of the plaintext encryption.
    On the other hand, this change w.r.t. OpenSSH also does not worsen
    any properties, as incorrect lengths will still trigger
    authentication failure for the overall packet (the plaintext length
    is implicitly authenticated by ChaCha20Poly1305).
  - A hash step is performed every 224\[21\] messages to rekey the the
    encryption ciphers, in order to provide forward security.

</ref> Because only fixed-length chunks (3-byte length fields) are
encrypted, we do not need to treat all length chunks as separate
messages. Instead, a single cipher (with the same nonce) is used for
multiple consecutive length fields. This avoids wasting 61 pseudorandom
bytes per packet, and makes the cost of having a separate cipher for
length encryption negligible.\[22\]

In order to provide forward security\[23\]\[24\], the encryption keys
for both plaintext and length encryption are cycled every 224 messages,
by switching to a new key that is generated by the key stream using the
old key.

#### Handshake: key exchange and version negotiation

Next we specify the handshake of a connection in detail.

As explained before, these messages are sent to set up the connection:

``` 
 ----------------------------------------------------------------------------------------------------
 | Initiator                         Responder                                                      |
 |                                                                                                  |
 | x, ellswift_X = ellswift_create(initiating=True)                                                 |
 |                                                                                                  |
 |           --- ellswift_X + initiator_garbage (initiator_garbage_len bytes; max 4095) --->        |
 |                                                                                                  |
 |                                   y, ellswift_Y = ellswift_create(initiating=False)              |
 |                                   ecdh_secret = v2_ecdh(                                         |
 |                                                     y, ellswift_X, ellswift_Y, initiating=False) |
 |                                   v2_initialize(initiator, ecdh_secret, initiating=False)        |
 |                                                                                                  |
 |           <-- ellswift_Y + responder_garbage (responder_garbage_len bytes; max 4095) +           |
 |                    responder_garbage_terminator (16 bytes) +                                     |
 |                    v2_enc_packet(initiator, b'', aad=responder_garbage) +                        |
 |                    v2_enc_packet(initiator, RESPONDER_TRANSPORT_VERSION) ---                     |
 |                                                                                                  |
 | ecdh_secret = v2_ecdh(x, ellswift_Y, ellswift_X, initiating=True)                                |
 | v2_initialize(responder, ecdh_secret, initiating=True)                                           |
 |                                                                                                  |
 |            --- initiator_garbage_terminator (16 bytes) +                                         |
 |                    v2_enc_packet(responder, b'', aad=initiator_garbage) +                        |
 |                    v2_enc_packet(responder, INITIATOR_TRANSPORT_VERSION) --->                    |
 |                                                                                                  |
 ----------------------------------------------------------------------------------------------------
```

##### Shared secret computation

The peers derive their shared secret through X-only ECDH, hashed
together with the exactly 64-byte public keys' encodings sent over the
wire.

    def v2_ecdh(priv, ellswift_theirs, ellswift_ours, initiating):
        ecdh_point_x32 = ellswift_ecdh_xonly(ellswift_theirs, priv)
        if initiating:
            # Initiating, place our public key encoding first.
            return sha256_tagged("bip324_ellswift_xonly_ecdh", ellswift_ours + ellswift_theirs + ecdh_point_x32)
        else:
            # Responding, place their public key encoding first.
            return sha256_tagged("bip324_ellswift_xonly_ecdh", ellswift_theirs + ellswift_ours + ecdh_point_x32)

Here, `sha256_tagged(tag, x)` returns a tagged hash value
`SHA256(SHA256(tag) || SHA256(tag) || x)` as in
[BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki#specification).

##### ElligatorSwift encoding of curve X coordinates

The functions `ellswift_create` and `ellswift_ecdh_xonly` encapsulate
the construction of ElligatorSwift-encoded public keys, and the
computation of X-only ECDH with ElligatorSwift-encoded public keys.

First we define a constant:

  - Let *c =
    0xa2d2ba93507f1df233770c2a797962cc61f6d15da14ecd47d8d27ae1cd5f852*.\[25\]

To define the needed functions, we first introduce a helper function,
matching the `XSwiftEC` function from the
[SwiftEC](https://eprint.iacr.org/2022/759.pdf) paper, instantiated for
the secp256k1 curve, with minor modifications. It maps pairs of integers
*(u, t)* (both in range *0..p-1*) to valid X coordinates on the curve.
Note that the specification here does not attempt to be constant time,
as it does not operate on secret data. In what follows, we use the
notation from
[BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki#specification).

  - *XSwiftEC(u, t)*:
      - Alter the inputs to guarantee an X coordinate on the
        curve:\[26\]
          - If *u mod p = 0*, let *u = 1* instead.
          - If *t mod p = 0*, let *t = 1* instead.
          - If *(u<sup>3</sup> + t<sup>2</sup> + 7) mod p = 0*, let *t =
            2t (mod p)* instead.
      - Let *X = (u<sup>3</sup> + 7 - t<sup>2</sup>)/(2t) (mod
        p).*\[27\]
      - Let *Y = (X + t)/(cu) (mod p)*.
      - For every *x* in *{u + 4Y<sup>2</sup>, (-X/Y - u)/2, (X/Y -
        u)/2}* (all *mod p*; the order matters):
          - If *lift\_x(x)* succeeds, return *x*. There is at least one
            such *x*.

To find encodings of a given X coordinate *x*, we first need the inverse
of *XSwiftEC*. The function *XSwiftECInv(x, u, case)* either returns *t*
such that *XSwiftEC(u, t) = x*, or *None*. The *case* variable is an
integer in range 0 to 7 inclusive, which selects which of the up to 8
valid such *t* values to return:

  - *XSwiftECInv(x, u, case)*:
      - If *case & 2 = 0*:
          - If *lift\_x(-x - u)* succeeds, return *None*.
          - Let *v = x* if *case & 1 = 0*; let *v = -x - u (mod p)*
            otherwise.
          - Let *s = -(u<sup>3</sup> + 7)/(u<sup>2</sup> + uv +
            v<sup>2</sup>) (mod p)*.
      - If *case & 2 = 2*:
          - Let *s = x - u (mod p)*.
          - If *s = 0*, return *None*.
          - Let *r* be the square root of *-s(4(u<sup>3</sup> + 7) +
            3u<sup>2</sup>s) (mod p).*\[28\] Return *None* if it does
            not exist.
          - If *case & 1 = 1*:
              - If *r = 0*, return *None*.
              - let *r = -r (mod p)*.
          - Let *v = (-u + r/s)/2*.
      - Let *w* be the square root of *s (mod p)*. Return *None* if it
        does not exist.
      - If *case & 4 = 4*, let *w = -w (mod p)*.
      - Return *w(u(c - 1)/2 - v)*.

The overall *XElligatorSwift* algorithm, matching the name used in the
paper, then uses this inverse to randomly''\[29\] sample encodings of
*x*:

  - *XElligatorSwift(x)*:
      - Loop:
          - Let *u* be a random non-zero integer in range *1..p-1*
            inclusive.
          - Let *case* be a random integer in range *0..7* inclusive.
          - Compute *t = XSwiftECInv(x, u, case)*.
          - If *t* is not *None*, return *(u, t)*. Otherwise, restart
            loop.

This is used to define the `ellswift_create` algorithm used in the
previous section; it generates a random private key, along with a
uniformly sampled 64-byte ElligatorSwift-encoded public key
corresponding to it:

  - *ellswift\_create()*:
      - Generate a random private key *priv* in range *1..p-1*.
      - Let *P = priv⋅G*, the corresponding public key point to *priv*.
      - Let *(u, t) = XElligatorSwift(x(P))*, an encoding of *x(P)*.
      - *ellswift\_pub = bytes(u) || bytes(t)*, its encoding as 64
        bytes.
      - Return *(priv, ellswift\_pub)*.

Finally the `ellswift_ecdh_xonly` algorithm is:

  - *ellswift\_ecdh\_xonly(ellswift\_theirs, priv)*:
      - Let *u = int(ellswift\_theirs\[:32\]) mod p*.
      - Let *t = int(ellswift\_theirs\[32:\]) mod p*.
      - Return *bytes(x(priv⋅lift\_x(XSwiftEC(u, t))))*.\[30\]

##### Keys and session ID derivation

The authenticated encryption construction proposed here requires two
32-byte keys per communication direction. These (in addition to a
session ID) are computed using HKDF\[31\] as specified in
[RFC 5869](https://tools.ietf.org/html/rfc5869) with SHA256 as the hash
function:

    def initialize_v2_transport(peer, ecdh_secret, initiating):
        # Include NETWORK_MAGIC to ensure a connection between nodes on different networks will immediately fail
        prk = HKDF_Extract(Hash=sha256, salt=b'bitcoin_v2_shared_secret' + NETWORK_MAGIC, ikm=ecdh_secret)
    
        peer.session_id = HKDF_Expand(Hash=sha256, PRK=prk, info=b'session_id', L=32)
    
        # Initialize the packet encryption ciphers.
        initiator_L = HKDF_Expand(Hash=sha256, PRK=prk, info=b'initiator_L', L=32)
        initiator_P = HKDF_Expand(Hash=sha256, PRK=prk, info=b'initiator_P', L=32)
        responder_L = HKDF_Expand(Hash=sha256, PRK=prk, info=b'responder_L', L=32)
        responder_P = HKDF_Expand(Hash=sha256, PRK=prk, info=b'responder_P', L=32)
        garbage_terminators = HKDF_Expand(Hash=sha256, PRK=prk, info=b'garbage_terminators', L=32)
        initiator_garbage_terminator = garbage_terminators[:16]
        responder_garbage_terminator = garbage_terminators[16:]
    
        if initiating:
            peer.send_L = FSChaCha20(initiator_L)
            peer.send_P = FSChaCha20Poly1305(initiator_P)
            peer.send_garbage_terminator = initiator_garbage_terminator
            peer.recv_L = FSChaCha20(responder_L)
            peer.recv_P = FSChaCha20Poly1305(responder_P)
            peer.recv_garbage_terminator = responder_garbage_terminator
        else:
            peer.send_L = FSChaCha20(responder_L)
            peer.send_P = FSChaCha20Poly1305(responder_P)
            peer.send_garbage_terminator = responder_garbage_terminator
            peer.recv_L = FSChaCha20(initiator_L)
            peer.recv_P = FSChaCha20Poly1305(initiator_P)
            peer.recv_garbage_terminator = initiator_garbage_terminator
    
        # To achieve forward secrecy we must wipe the key material used to initialize the ciphers:
        memory_cleanse(ecdh_secret, prk, initiator_L, initiator_P, responder_L, responder_K)

The session ID uniquely identifies the encrypted channel. v2 clients
supporting this proposal may present the entire session ID (encoded as a
hex string) to the node operator to allow for manual, out of band
comparison with the peer node operator. Future transport versions may
introduce optional authentication methods that compare the session ID as
seen by the two endpoints in order to bind the encrypted channel to the
authentication.

##### Overall handshake pseudocode

To establish a v2 encrypted connection, the initiator generates an
ephemeral secp256k1 keypair and sends an unencrypted ElligatorSwift
encoding of the public key to the responding peer followed by
unencrypted pseudorandom bytes `initiator_garbage` of length
`garbage_len < 4096`.

    def initiate_v2_handshake(peer, garbage_len):
        peer.privkey_ours, peer.ellswift_ours = ellswift_create(initiating=True)
        peer.sent_garbage = rand_bytes(garbage_len)
        send(peer, peer.ellswift_ours + peer.sent_garbage)

The responder generates an ephemeral keypair for itself and derives the
shared ECDH secret (using the first 64 received bytes) which enables it
to instantiate the encrypted transport. It then sends 64 bytes of the
unencrypted ElligatorSwift encoding of its own public key and its own
`responder_garbage` also of length `garbage_len < 4096`. If the first 12
bytes received match the v1 prefix, the v1 protocol is used instead.

    TRANSPORT_VERSION = b''
    NETWORK_MAGIC = b'\xf9\xbe\xb4\xd9' # Mainnet network magic; differs on other networks.
    V1_PREFIX = NETWORK_MAGIC + b'version\x00'
    
    def respond_v2_handshake(peer, garbage_len):
        peer.received_prefix = b""
        while len(peer.received_prefix) < 12:
            peer.received_prefix += receive(peer, 1)
            if peer.received_prefix[-1] != V1_PREFIX[len(peer.received_prefix) - 1]:
                peer.privkey_ours, peer.ellswift_ours = ellswift_create(initiating=False)
                peer.sent_garbage = rand_bytes(garbage_len)
                send(peer, ellswift_Y + peer.sent_garbage)
                return
        use_v1_protocol()

Upon receiving the encoded responder public key, the initiator derives
the shared ECDH secret and instantiates the encrypted transport. It then
sends the derived 16-byte `initiator_garbage_terminator` followed by an
authenticated, encrypted packet with empty contents\[32\] to
authenticate the garbage, and its own version packet. It then receives
the responder's garbage and garbage authentication packet (delimited by
the garbage terminator), and checks if the garbage is authenticated
correctly. The responder performs very similar steps, but includes the
earlier received prefix bytes in the public key. As mentioned before,
the encrypted packets for the **version negotiation phase** can be
piggybacked with the garbage authentication packet to minimize
roundtrips.

    def complete_handshake(peer, initiating):
        received_prefix = b'' if initiating else peer.received_prefix
        ellswift_theirs = receive(peer, 64 - len(received_prefix))
        ecdh_secret = v2_ecdh(peer.privkey_ours, ellswift_theirs, peer.ellswift_ours,
                              initiating=initiating)
        initialize_v2_transport(peer, ecdh_secret, initiating=True)
        # Send garbage terminator + garbage authentication packet + version packet.
        send(peer, peer.send_garbage_terminator +
                   v2_enc_packet(peer, b'', aad=peer.sent_garbage) +
                   v2_enc_packet(peer, TRANSPORT_VERSION))
        # Skip garbage, until encountering garbage terminator.
        received_garbage = recv(peer, 16)
        for i in range(4096):
            if received_garbage[-16:] == peer.recv_garbage_terminator:
                # Receive, decode, and ignore garbage authentication packet (decoy or not)
                v2_receive_packet(peer, aad=received_garbage, skip_decoy=False)
                # Receive, decode, and ignore version packet, skipping decoys
                v2_receive_packet(peer)
                return
            else:
                received_garbage += recv(peer, 1)
        # Garbage terminator was not seen after 4 KiB of garbage.
        disconnect(peer)

#### Packet encryption

Lastly, we specify the packet encryption cipher in detail.

##### Existing cryptographic primitives

Packet encryption is built on two existing primitives:

  - **ChaCha20Poly1305** is specified as `AEAD_CHACHA20_POLY1305` in
    [RFC 8439
    section 2.8](https://datatracker.ietf.org/doc/html/rfc8439#section-2.8).
    It is an authenticated encryption protocol with associated data
    (AEAD), taking a 256-bit key, 96-bit nonce, and an arbitrary-length
    byte array of associated authenticated data (AAD). Due to the
    built-in authentication tag, ciphertexts are 16 bytes longer than
    the corresponding plaintext. In what follows:
      - `aead_chacha20_poly1305_encrypt(key, nonce, aad, plaintext)`
        refers to a function that takes as input a 32-byte array *key*,
        a 12-byte array *nonce*, an arbitrary-length byte array *aad*,
        and an arbitrary-length byte array *plaintext*, and returns a
        byte array *ciphertext*, 16 bytes longer than the plaintext.
      - `aead_chacha20_poly1305_decrypt(key, nonce, aad, ciphertext)`
        refers to a function that takes as input a 32-byte array *key*,
        a 12-byte array *nonce*, an arbitrary-length byte array *aad*,
        and an arbitrary-length byte array *ciphertext*, and returns
        either a byte array *plaintext* (16 bytes shorter than the
        ciphertext), or *None* in case the ciphertext was not a valid
        ChaCha20Poly1305 encryption of any plaintext with the specified
        *key*, *nonce*, and *aad*.
  - The **ChaCha20 Block Function** is specified in [RFC 8439
    section 2.3](https://datatracker.ietf.org/doc/html/rfc8439#section-2.8).
    It is a pseudorandom function (PRF) taking a 256-bit key, 96-bit
    nonce, and 32-bit counter, and outputs 64 pseudorandom bytes. It is
    the underlying building block on which ChaCha20 (and ultimately,
    ChaCha20Poly1305) is built. In what follows:
      - `chacha20_block(key, nonce, count)` refers to a function that
        takes as input a 32-byte array *key*, a 12-byte array *nonce*,
        and an integer *count* in range *0..2<sup>32</sup>-1*, and
        returns a byte array of length 64.

These will be used for plaintext encryption and length encryption,
respectively.

##### Rekeying wrappers: FSChaCha20Poly1305 and FSChaCha20

To provide re-keying every 224 packets, we specify two wrappers.

The first is **FSChaCha20Poly1305**, which represents a ChaCha20Poly1305
AEAD, which automatically changes the nonce after every message, and
rekeys every 224 messages by encrypting 32 zero bytes\[33\], and using
the first 32 bytes of the result. Each message will be used for one
packet. Note that in our protocol, any FSChaCha20Poly1305 instance is
always either exclusively encryption or exclusively decryption, as
separate instances are used for each direction of the protocol. The
nonce used for a message is composed of the 32-bit little endian
encoding of the number of messages with the current key, followed by the
64-bit little endian encoding of the number of rekeyings performed. For
rekeying, the first 32-bit integer is set to *0xffffffff*.

    REKEY_INTERVAL = 224
    
    class FSChaCha20Poly1305:
        """Rekeying wrapper AEAD around ChaCha20Poly1305."""
    
        def __init__(self, initial_key):
            self.key = initial_key
            self.packet_counter = 0
    
        def crypt(self, aad, text, is_decrypt):
            nonce = ((self.packet_counter % REKEY_INTERVAL).to_bytes(4, 'little') +
                     (self.packet_counter // REKEY_INTERVAL).to_bytes(8, 'little'))
            if is_decrypt:
                ret = aead_chacha20_poly1305_decrypt(self.key, nonce, aad, text)
            else:
                ret = aead_chacha20_poly1305_encrypt(self.key, nonce, aad, text)
            if (self.packet_counter + 1) % REKEY_INTERVAL == 0:
                rekey_nonce = b"\xFF\xFF\xFF\xFF" + nonce[4:]
                self.key = aead_chacha20_poly1305_encrypt(self.key, rekey_nonce, b"", b"\x00" * 32)[:32]
            self.packet_counter += 1
            return ret
    
        def decrypt(self, aad, ciphertext):
            return self.crypt(aad, ciphertext, True)
    
        def encrypt(self, aad, plaintext):
            return self.crypt(aad, plaintext, False)

The second is **FSChaCha20**, a (single) stream cipher which is used for
the lengths of all packets. Encryption and decryption are identical
here, so a single function `crypt` is exposed. It XORs the input with
bytes generated using the ChaCha20 block function, rekeying every 224
chunks using the next 32 bytes of the block function output as new key.
A *chunk* refers here to a single invocation of `crypt`. As explained
before, the same cipher is used for 224 consecutive chunks, to avoid
wasting cipher output. The nonce used for these batches of 224 chunks is
composed of 4 zero bytes followed by the 64-bit little endian encoding
of the number of rekeyings performed. The block counter is reset to 0
after every rekeying.

    class FSChaCha20:
        """Rekeying wrapper stream cipher around ChaCha20."""
    
        def __init__(self, initial_key):
            self.key = initial_key
            self.block_counter = 0
            self.chunk_counter = 0
            self.keystream = b''
    
        def get_keystream_bytes(self, nbytes):
            while len(self.keystream) < nbytes:
                nonce = ((0).to_bytes(4, 'little') +
                         (self.chunk_counter // REKEY_INTERVAL).to_bytes(8, 'little'))
                self.keystream += chacha20_block(self.key, nonce, self.block_counter)
                self.block_counter += 1
            ret = self.keystream[:nbytes]
            self.keystream = self.keystream[nbytes:]
            return ret
    
        def crypt(self, chunk):
            ks = self.get_keystream_bytes(len(chunk))
            ret = bytes([ks[i] ^ chunk[i] for i in range(len(chunk))])
            if ((self.chunk_counter + 1) % REKEY_INTERVAL) == 0:
                self.key = self.get_keystream_bytes(32)
                self.block_counter = 0
            self.chunk_counter += 1
            return ret

##### Overall packet encryption and decryption pseudocode

Encryption and decryption of packets then follow by composing the
ciphers from the previous section as building blocks.

    LENGTH_FIELD_LEN = 3
    HEADER_LEN = 1
    IGNORE_BIT_POS = 7
    
    def v2_enc_packet(peer, contents, aad=b'', ignore=False):
        assert len(contents) <= 2**24 - 1
        header = (ignore << IGNORE_BIT_POS).to_bytes(HEADER_LEN, 'little')
        plaintext = header + contents
        aead_ciphertext = peer.send_P.encrypt(aad, plaintext)
        enc_contents_len = peer.send_L.encrypt(len(contents).to_bytes(LENGTH_FIELD_LEN, 'little'))
        return enc_contents_len + aead_ciphertext

    CHACHA20POLY1305_EXPANSION = 16
    
    def v2_receive_packet(peer, aad=b'', skip_decoy=True):
        while True:
            enc_contents_len = receive(peer, LENGTH_FIELD_LEN)
            contents_len = int.from_bytes(peer.recv_L.crypt(enc_contents_len), 'little')
            aead_ciphertext = receive(peer, HEADER_LEN + contents_len + CHACHA20POLY1305_EXPANSION)
            plaintext = peer.recv_P.decrypt(aead_ciphertext)
            if plaintext is None:
                disconnect(peer)
                break
            header = plaintext[:HEADER_LEN]
            if not (skip_decoy and header[0] & (1 << IGNORE_BIT_POS)):
                return plaintext[HEADER_LEN:]

#### Performance

Each v1 P2P message uses a double-SHA256 checksum truncated to 4 bytes.
Roughly the same amount of computation power is required for encrypting
and authenticating a v2 P2P message as proposed.

### Application layer specification

#### v2 Bitcoin P2P message structure

v2 Bitcoin P2P transport layer packets use the encrypted message
structure shown above. An unencrypted application layer **contents** is
composed of:

| Field             | Size in bytes    | Comments                                                            |
| ----------------- | ---------------- | ------------------------------------------------------------------- |
| `message_type`    | *1..13*          | either a one byte ID or an ASCII string prefixed with a length byte |
| `message_payload` | `message_length` | message payload                                                     |

If the first byte of `message_type` is in the range *1..12*, it is
interpreted as the number of ASCII bytes that follow for the message
type. If it is in the range *13..255*, it is interpreted as a message
type ID. This structure results in smaller messages than the v1 protocol
as most messages sent/received will have a message type ID.\[34\]

The following table lists currently defined message type IDs:

|      | 0                | 1               | 2                | 3                |
| ---- | ---------------- | --------------- | ---------------- | ---------------- |
| \+0  | (undefined)      | (1 byte string) | (2 byte string)  | (3 byte string)  |
| \+4  | (4 byte string)  | (5 byte string) | (6 byte string)  | (7 byte string)  |
| \+8  | (8 byte string)  | (9 byte string) | (10 byte string) | (11 byte string) |
| \+12 | (12 byte string) | `ADDR`          | `BLOCK`          | `BLOCKTXN`       |
| \+16 | `CMPCTBLOCK`     | `FEEFILTER`     | `FILTERADD`      | `FILTERCLEAR`    |
| \+20 | `FILTERLOAD`     | `GETADDR`       | `GETBLOCKS`      | `GETBLOCKTXN`    |
| \+24 | `GETDATA`        | `GETHEADERS`    | `HEADERS`        | `INV`            |
| \+28 | `MEMPOOL`        | `MERKLEBLOCK`   | `NOTFOUND`       | `PING`           |
| \+32 | `PONG`           | `SENDCMPCT`     | `SENDHEADERS`    | `TX`             |
| \+36 | `VERACK`         | `VERSION`       | `GETCFILTERS`    | `CFILTER`        |
| \+40 | `GETCFHEADERS`   | `CFHEADERS`     | `GETCFCHECKPT`   | `CFCHECKPT`      |
| \+44 | `WTXIDRELAY`     | `ADDRV2`        | `SENDADDRV2`     | `SENDTXRCNCL`    |
| \+48 | `REQRECON`       | `SKETCH`        | `REQSKETCHEXT`   | `RECONCILDIFF`   |
| ≥52  | (undefined)      |                 |                  |                  |

The message types may be updated separately after BIP finalization.

### Signaling specification

#### Signaling v2 support

Peers supporting the v2 transport protocol signal support by advertising
the `NODE_P2P_V2 = (1 << 11)` service flag in addr relay. If met with
immediate disconnection when establishing a v2 connection, clients
implementing this proposal are encouraged to retry connecting using the
v1 protocol.\[35\]

## Test Vectors

For development and testing purposes, we provide a collection of test
vectors in CSV format, and a naive, highly inefficient, [reference
implementation](bip-0324/reference.py "wikilink") of the relevant
algorithms. This code is for demonstration purposes only:

  - [XElligatorSwift
    vectors](bip-0324/xelligatorswift_test_vectors.csv "wikilink") give
    examples of ElligatorSwift-encoded public keys, and the X coordinate
    they map to.
  - [XSwiftEC vectors](bip-0324/xswiftec_test_vectors.csv "wikilink")
    give examples of *(u, x)* pairs, and the various *t* values that
    *xswiftec\_inv* maps them to.
  - [Packet encoding
    vectors](bip-0324/packet_encoding_test_vectors.csv "wikilink")
    illustrate the lifecycle of the authenticated encryption scheme
    proposed in this document.

## Rationale and References

<references/>

## Acknowledgements

Thanks to everyone (last name order) that helped invent and develop the
ideas in this proposal:

  - Matt Corallo
  - Lloyd Fournier
  - Gregory Maxwell

<!-- end list -->

1.  **Why is it a bad idea to have nodes exclusively connected over
    Tor?** See the [Bitcoin over Tor isn't a Good
    Idea](https://arxiv.org/abs/1410.6079) paper
2.  **Do other protocols not support exporting a session ID?** While
    [Noise](https://noiseprotocol.org/noise.html#channel-binding) and
    [TLS (as a
    draft)](https://datatracker.ietf.org/doc/draft-ietf-kitten-tls-channel-bindings-for-tls13/)
    offer similar protocol extensions for exporting session IDs, using
    channel binding for authentication is not at the focus of their
    design and would not avoid the bulk of additional complexity due to
    the native support of authentication methods.
3.  **How can shapability help circumvent fragmentation-pattern based
    censoring?** See [this Tor
    issue](https://gitlab.torproject.org/legacy/trac/-/issues/20348#note_2229522)
    as an example.
4.  **What is ElligatorSwift and why use it?** The [SwiftEC
    paper](https://eprint.iacr.org/2022/759.pdf) describes a method
    called ElligatorSwift which allows encoding elliptic curve points in
    a way that is indistinguishable from a uniformly distributed
    bitstream. While a random 256-bit string has about 50% chance of
    being a valid X coordinate on the secp256k1 curve, every 512-bit
    string is a valid ElligatorSwift encoding of a curve point, making
    the encoded point indistinguishable from random when using an
    encoder that can sample uniformly.
5.  **How fast is ElligatorSwift?** Our benchmarks show that
    ElligatorSwift encoded ECDH is about 50% more expensive than
    unencoded ECDH. Given the fast performance of ECDH and the low
    frequency of new connections, we found the performance trade-off
    acceptable for the pseudorandom bytestream and future censorship
    resistance it can enable.
6.  **How was the limit of 4095 bytes garbage chosen?** It is a balance
    between having sufficient freedom to hide information, and allowing
    it to be large enough so that the necessary 64 bytes of public key
    is small compared to it on the one hand, and bandwidth waste on the
    other hand.
7.  **Why does the affordance for garbage exist in the protocol?** The
    garbage strings after the public keys are needed for shapability of
    the handshake. Neither peer can send decoy packets before having
    received at least the other peer's public key, i.e., neither peer
    can send more than 64 bytes before having received 64 bytes.
8.  **What if a v2 initiator's public key starts accidentally with these
    12 bytes?** This is so unlikely (probability of *2<sup>-96</sup>*)
    to happen randomly in the v2 protocol that the initiator does not
    need to specifically avoid it.
9.  Bitcoin Core versions \<=0.4.0 and \>=22.0 ignore valid P2P messages
    that are received prior to a VERSION message. Bitcoin Core versions
    between 0.4.0 and 22.0 assign a misbehavior score to the peer upon
    receiving such messages. v2 clients implementing this proposal will
    interpret any message other than VERSION received as the first
    message to be the initiation of a v2 connection, and will result in
    disconnection for v1 initiators that send any message type other
    than VERSION as the first message. We are not aware of any
    implementations where this could pose a problem.
10. **Why use X-only ECDH?** Using only the X coordinate provides the
    same security as using a full encoding of the secret curve point but
    allows for more efficient implementation by avoiding the need for
    square roots to compute Y coordinates.
11. **Why is the shared secret computation a function of the exact
    64-byte public encodings sent?** This makes sure that an attacker
    cannot modify the public key encoding used without modifying the
    rest of the stream. If a third party wants the ability to modify
    stream bytes, they need to perform a full MitM attack on the
    connection.
12. **What length is sufficient for garbage terminators?** The length of
    the garbage terminators determines the probability of accidental
    termination of a legitimate v2 connection due to garbage bytes (sent
    prior to ECDH) inadvertently including the terminator. 16 byte
    terminators with 4095 bytes of garbage yield a negligible
    probability of such collision which is likely orders of magnitude
    lower than random connection failure on the Internet.
13. **Why does the protocol need a garbage terminator?** While it is in
    principle possible to use the garbage authentication packet directly
    as a terminator (scan until a valid authentication packet follows),
    this would be significantly slower than just scanning for a fixed
    byte sequence, as it would require recomputing a Poly1305 tag after
    every received byte.
14. **Why does the protocol require a garbage authentication packet?**
    Otherwise the garbage would be modifiable by a third party without
    consequences. We want to force any active attacker to have to
    maintain a full protocol state. In addition, such malleability
    without the consequence of connection termination could enable
    protocol fingerprinting.
15. **What features could be added in future protocol versions?**
    Examples of features that could be added in future versions include
    post-quantum cryptography upgrades to the handshake, and optional
    authentication.
16. **How will future versions encode version numbers in the version
    packet?** Future versions could, for example, specify that the
    contents of the version packet is to be interpreted as an integer
    version number (with empty representing 0), and if the minimum of
    both numbers is N, that being interpreted as choosing a "v2.N"
    protocol version. Alternatively, certain bytes of the version packet
    contents could be interpreted as a bitvector of optional features.
17. **Is *2<sup>24</sup>-1* bytes sufficient as maximum content size?**
    The current Bitcoin P2P protocol has no messages which support more
    than 4000000 bytes of application payload. By supporting up to
    *2<sup>24</sup>-1* we can accommodate future evolutions needing more
    than 4 times that value. Hypothetical protocol changes that have
    even more data to exchange than that should probably use multiple
    separate messages anyway, because of the per-peer receive buffer
    sizes involved, and the inability to start processing a message
    before it is fully received. Of course, future versions of the
    transport protocol could change the size of the length field, if
    this were really needed.
18. **Why is the header a part of the plaintext and not included
    alongside the length field?** The packet length field is the minimum
    information that must be available before we can leverage the
    standard RFC8439 AEAD. Any other data, including metadata like the
    header being in the content encryption makes it easier to reason
    about the protocol security w.r.t. data being used before it is
    authenticated. If the ignore bit was not part of the content,
    another mechanism would be needed to authenticate it; for example,
    it could be fed as AAD to the AEAD cipher. We feel the complexity of
    such an approach outweighs the benefit of saving one byte per
    message.
19. **Why is ChaCha20Poly1305 chosen as basis for packet encryption?**
    It is a very widely used authenticated encryption cipher (used
    amongst others in SSH, TLS 1.2, TLS 1.3,
    [QUIC](https://en.wikipedia.org/wiki/QUIC), Noise, and
    [WireGuard](https://www.wireguard.com/protocol/); in the latter it
    is currently even the only supported cipher), with very good
    performance in general purpose software implementations. While
    AES-based ciphers (including the winners in the
    [CAESAR](https://competitions.cr.yp.to/caesar.html) competition in
    non-lightweight categories) perform significantly better on systems
    with AES hardware acceleration, they are also significantly slower
    in pure software implementations. We choose to optimize for the
    weakest hardware.
20. **Why is the length encryption not separately authenticated?**
    Informally, the relevant security goal we aim for is to hide the
    number of packets and their lengths (i.e., the packet boundaries)
    against a passive attacker that receives the bytestream without
    timing or fragmentation information. (A formal definition can be
    found for example in [Hansen 2016
    (Definition 22)](https://himsen.github.io/pdf/thesis.pdf) under the
    name "boundary hiding against chosen-plaintext attacks (BH-CPA)".)
    However, we do not aim to hide packet boundaries against active
    attackers because active attackers can always exploit the fact that
    the Bitcoin P2P protocol is largely query-response based: they can
    trickle the bytes on the stream one-by-one unmodified and observe
    when a response comes (see [Hansen 2016
    (Section 3.9)](https://himsen.github.io/pdf/thesis.pdf) for a
    in-depth discussion). With that in mind, we accept that an active
    (non-MitM) attacker is able to figure out some information about
    packet boundaries by flipping certain bits in the unauthenticated
    length field, and observing the other side disconnecting immediately
    or later. Thus, we choose to use unauthenticated encryption for the
    length data, which is sufficient to achieve boundary hiding against
    passive attackers, and saves 16 bytes of bandwidth per packet.
21. **How was the rekeying interval 224 chosen?** Assuming a node sends
    only ping messages every 20 minutes (the timeout interval for
    post-[BIP31](https://github.com/bitcoin/bips/blob/master/bip-0031.mediawiki)
    connections) on a connection, the node will transmit 224 packets in
    about 3.11 days. This means *soft rekeying* after a fixed number of
    packets automatically translates to an upper-bound of time interval
    for rekeying, while being much simpler to coordinate than an actual
    time-based rekeying regime. At the same time, doing it once every
    224 messages is sufficiently infrequent that it has only negligible
    impact on performance. Furthermore, 224 times 3 bytes (the number of
    bytes consumed by each length encryption) is 672, which is a
    multiple of 64 minus 32. This means that at the end of 224 length
    encryptions, exactly 32 bytes of keystream data remain that can be
    used as next key.
22. **Is it acceptable to use a less standard construction for length
    encryption?** The fact that multiple (non-overlapping) bytes
    generated by a single ChaCha20 cipher are used for the encryption of
    multiple consecutive length fields is uncommon. We feel the
    performance cost gained by this deviation is worth it (especially
    for small packets, which are very common in Bitcoin's P2P protocol),
    given the low guarantees that are feasible for length encryption in
    the first place, and the result is still sufficient to provide
    pseudorandomness from the view of passive attackers. For plaintext
    encryption, we independently use a very standard construction, as
    the stakes for confidentiality and integrity there are much higher.
23. **What value does forward security provide?** Re-keying ensures
    [forward secrecy within a
    session](https://eprint.iacr.org/2001/035.pdf), i.e., an attacker
    compromising the current session secrets cannot derive past
    encryption keys in the same session.
24. **Why have a cipher with forward secrecy but no periodical refresh
    of the ECDH key exchange?** Our cipher ratchets encryption keys
    forward in order to protect messages encrypted under *past*
    encryption keys. In contrast, re-performing ECDH key exchange would
    protect messages encrypted under *future* encryption keys, i.e., it
    would re-establish security after the attacker had compromised one
    of the peers *temporarily* (e.g., the attacker obtains a memory
    dump). We do not believe protecting against that is a priority: an
    attacker that, for whatever reason, is capable of an attack that
    reveals encryption keys (or other session secrets) of a peer once is
    likely capable of performing the same attack again after peers have
    re-performed the ECDH key exchange. Thus, we do not believe the
    benefits of re-performing key exchange outweigh the additional
    complexity that comes with the necessary coordination between the
    peers. We note that the initiator could choose to close and re-open
    the entire connection in order to force a refresh of the ECDH key
    exchange, but that introduces other issues: a connection slot needs
    to be kept open at the responder side, it is not cryptographically
    guaranteed that really the same initiator will use it, and the
    observable TCP reset and handshake may create a detectable pattern.
25. **What is the *c* constant used in *XSwiftEC*?** The algorithm
    requires a constant *√-3 (mod p)*; in other words, a number *c* such
    that *-c<sup>2</sup> mod p = 3*. There are two solutions to this
    equation, one which is itself a square modulo *p*, and its negation.
    We choose the square one.
26. **Why do the inputs to the XSwiftEC algorithm need to be altered?**
    This step deviates from the paper, which maps a negligibly small
    subset of inputs (around *3/2<sup>256</sup>*) to the point at
    infinity. To avoid the need to deal with the case where a peer could
    craft encodings that intentionally trigger this edge case, we remap
    them to inputs that yield a valid X coordinate.
27. **What does the division (/) sign in modular arithmetic refer to?**
    Note that the division in these expressions corresponds to
    multiplication with the modular inverse modulo *p*, i.e. *a / b (mod
    p)* with nonzero *b* is the unique solution *x* for which *bx = a
    (mod p)*. It can be computed as *ab<sup>p-2</sup> (mod p)*, but more
    efficient algorithms exist.
28. **How to compute a square root mod *p*?** Due to the structure of
    *p*, a candidate for the square root of *a* mod *p* can be computed
    as *x = a<sup>(p+1)/4</sup> mod p*. If *a* is not a square mod *p*,
    this formula returns the square root of *-a mod p* instead, so it is
    necessary to verify that *x<sup>2</sup> mod p = a*. If that is the
    case *-x mod p* is a solution too, but we define "the" square root
    to be equal to that expression (the square root will therefore
    always be a square itself, as *(p+1)/4* is even). This algorithm is
    a specialization of the [Tonelli-Shanks
    algorithm](https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm).
29. **Can the ElligatorSwift encoding be used to construct public key
    encodings that satisfy a certain structure (and not pseudorandom)?**
    The algorithm chooses the first 32 bytes (i.e., the value *u*) and
    then computes a corresponding *t* such that the mapping to the curve
    point holds. In general, picking *u* from a uniformly random
    distribution provides pseudorandomness. But we can also fix any of
    the 32 bytes in *u*, and the algorithm will still find a
    corresponding *t*. The fact that it is possible to fix the first 32
    bytes, combined with the garbage bytes in the handshake, provides a
    limited but very simple method of parroting other protocols such as
    [TLS 1.3](https://tls13.xargs.org/), which can be deployed by one of
    the peers without explicit support from the other peer. More general
    methods of parroting, e.g., introduced by defining new protocol or a
    protocol upgrade, are not precluded.
30. **Does it matter which point *lift\_x* maps to?** Either point is
    valid, as they are negations of each other, and negations do not
    affect the output X coordinate.
31. **Why use HKDF for deriving key material?** The shared secret
    already involves a hash function to make sure the public key
    encodings contribute to it, which negates some of the need for HKDF
    already. We still use it as it is the standard mechanism for
    deriving many keys from a single secret, and its computational cost
    is low enough to be negligible compared to the rest of a connection
    setup.
32. **Does the content of the garbage authentication packet need to be
    empty?** The receiver ignores the content of the garbage
    authentication packet, so its content can be anything, and it can in
    principle be used as a shaping mechanism too. There is however no
    need for that, as immediately afterwards the initiator can start
    using decoy packets as (much more flexible) shaping mechanism
    instead.
33. **Why is rekeying implemented in terms of an invocation of the
    AEAD?** This means the FSChaCha20Poly1305 wrapper can be thought of
    as a pure layer around the ChaCha20Poly1305 AEAD. Actual
    implementations can take advantage of the fact that this formulation
    is equivalent to using byte 64 through 95 of the keystream output of
    the underlying ChaCha20 cipher as new key, avoiding the need for
    Poly1305 in the process.
34. **How do the length between v1 and v2 compare?** For messages that
    use the 1-byte short message type ID, v2 packets use 3 bytes less
    per message than v1.
35. **Why are v2 clients met with immediate disconnection encouraged to
    retry with a v1 connection?** Service flags propagated through
    untrusted intermediaries using ADDR and ADDRV2 P2P messages and are
    OR'ed when received from multiple sources. An untrusted intermediary
    could falsely advertise a potential peer as supportive of v2
    connections. Connection downgrades to v1 mitigate the risk of a
    network participant being blackholed via false advertising.