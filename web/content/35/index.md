+++
title = "mempool message"
date = 2012-08-16
weight = 35
in_search_index = true

[extra]
bip = 35
status = "Final"
github = "https://github.com/bitcoin/bips/blob/master/bips"
+++

      BIP: 35
      Layer: Peer Services
      Title: mempool message
      Author: Jeff Garzik <jgarzik@exmulti.com>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0035
      Status: Final
      Type: Standards Track
      Created: 2012-08-16

## Abstract

Make a network node's transaction memory pool accessible via a new
"mempool" message. Extend the existing "getdata" message behavior to
permit accessing the transaction memory pool.

## Motivation

Several use cases make it desireable to expose a network node's
transaction memory pool:

1.  SPV clients, wishing to obtain zero-confirmation transactions sent
    or received.
2.  Miners, to avoid missing lucrative fees, downloading existing
    network transactions after a restart.
3.  Remote network diagnostics.

## Specification

1.  The mempool message is defined as an empty message where pchCommand
    == "mempool"
2.  Upon receipt of a "mempool" message, the node will respond with an
    "inv" message containing MSG\_TX hashes of all the transactions in
    the node's transaction memory pool, if any.
3.  The typical node behavior in response to an "inv" is "getdata".
    However, the reference Satoshi implementation ignores requests for
    transaction hashes outside that which is recently relayed. To
    support "mempool", an implementation must extend its "getdata"
    message support to querying the memory pool.
4.  Feature discovery is enabled by checking two "version" message
    attributes:
    1.  Protocol version &gt;= 60002
    2.  NODE\_NETWORK bit set in nServices

Note that existing implementations drop "inv" messages with a vector
size &gt; 50000.

## Backward compatibility

Older clients remain 100% compatible and interoperable after this
change.

## Implementation

<https://github.com/bitcoin/bitcoin/pull/1641>
