
+++
title = "mempool message"
date = 2012-08-16
weight = 35

[taxonomies]
authors = ["Jeff Garzik"]
status = ["Final"]

[extra]
bip = 35
status = ["Final"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0035.mediawiki"
note = "THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING"
+++

```
  BIP: 35
  Layer: Peer Services
  Title: mempool message
  Author: Jeff Garzik <jgarzik@exmulti.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0035
  Status: Final
  Type: Standards Track
  Created: 2012-08-16
```

<h2>Abstract</h2>


Make a network node's transaction memory pool accessible via a new "mempool" message.  Extend the existing "getdata" message behavior to permit accessing the transaction memory pool.

<h2>Motivation</h2>


Several use cases make it desirable to expose a network node's transaction memory pool:
1.  SPV clients, wishing to obtain zero-confirmation transactions sent or received.
1.  Miners, to avoid missing lucrative fees, downloading existing network transactions after a restart.
1.  Remote network diagnostics.


<h2>Specification</h2>


1.  The mempool message is defined as an empty message where pchCommand == "mempool"
1.  Upon receipt of a "mempool" message, the node will respond with an "inv" message containing MSG_TX hashes of all the transactions in the node's transaction memory pool, if any.
1.  The typical node behavior in response to an "inv" is "getdata". However, the reference Satoshi implementation ignores requests for transaction hashes outside that which is recently relayed. To support "mempool", an implementation must extend its "getdata" message support to querying the memory pool.
1.  Feature discovery is enabled by checking two "version" message attributes:
    1.  Protocol version >= 60002
    1.  NODE_NETWORK bit set in nServices


Note that existing implementations drop "inv" messages with a vector size > 50000.

<h2>Backward compatibility</h2>


Older clients remain 100% compatible and interoperable after this change.

<h2>Implementation</h2>


https://github.com/bitcoin/bitcoin/pull/1641
