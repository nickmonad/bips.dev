+++
title = "Hierarchical Deterministic Script Templates"
date = 2015-11-20
weight = 124
in_search_index = true

[extra]
bip = 124
status = "Rejected"
github = "https://github.com/bitcoin/bips/blob/master/bips"
+++

      BIP: 124
      Layer: Applications
      Title: Hierarchical Deterministic Script Templates
      Author: Eric Lombrozo <eric@ciphrex.com>
              William Swanson <swansontec@gmail.com>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0124
      Status: Rejected
      Type: Informational
      Created: 2015-11-20
      License: PD
      Post-History: http://lists.linuxfoundation.org/pipermail/bitcoin-dev/2015-November/011795.html

## Abstract

This BIP defines a script template format that can be used by wallets to
deterministically generate scripts with specific authorization policies
using the key derivation mechanism defined in BIP32.

## Motivation

Currently existing wallets typically issue scripts in only a tiny
handful of widely used formats. The most popular formats are
pay-to-pubkey-hash and m-of-n pay-to-script-hash (BIP16). However,
different wallets tend to use mutually incompatible derivation schemes
to generate signing keys and construct scripts from them. Moreover, with
the advent of hashlocked and timelocked contracts (BIP65, BIP112), it is
necessary for different wallets to be able to cooperatively generate
even more sophisticated scripts.

In addition, there's a lot of ongoing work in the development of
multilayered protocols that use the blockchain as a settlement layer
(i.e. the Lightning Network). These efforts require sufficiently
generalized templates to allow for rapidly evolving script designs.

This BIP provides a generalized format for constructing a script
template that guarantees that different wallets will all produce the
same scripts for a given set of derivation paths according to BIP32.

## Specification

### Keys

An individual key is determined by a BIP32 derivation path and an index.
For convenience, we introduce the following notation:

**A**<sub>k</sub> = (derivation path for A)/k

### Key Groups

Let **m**<sub>i</sub> denote distinct BIP32 derivation paths. We define
a key group of n keys as a set of key derivation paths with a free index
k:

{**K**<sub>k</sub>} = { **m**<sub>1</sub>/k, **m**<sub>2</sub>/k,
**m**<sub>3</sub>/k, ..., **m**<sub>n</sub>/k }

Key groups are useful for constructing scripts that are symmetric in a
set of keys. Scripts are symmetric in a set of keys if the semantics of
the script is unaffected by permutations of the keys. Key groups enforce
a canonical form and can improve privacy.

### Sorting

We define a lexicographic sorting of the keys. (TODO: specification of
sorting conventions - compressed pubkeys, encoding, etc...)

Define {**K**<sub>k</sub>}<sub>j</sub> to be the jth element of the
sorted keys for derivation index k.

### Script Templates

We construct script templates by inserting placeholders for data into a
script. To denote a placeholder, we use the following notation:

*Script*(**A**) = opcodes \[**A**\] opcodes

We extend this notation to an arbitrary number of placeholders:

*Script*(**X1**, **X2**, ..., **Xn**) = opcodes \[**X1**\] opcodes
\[**X2**\] opcodes ... opcodes \[**Xn**\] opcodes

We introduce the following convenient notation for sorted key groups:

\[{**K**<sub>k</sub>}\] = \[{**K**<sub>k</sub>}<sub>1</sub>\]
\[{**K**<sub>k</sub>}<sub>2</sub>\] ...
\[{**K**<sub>k</sub>}<sub>n</sub>\]

### Operations on Keys

In some applications, we might want to insert the result of some
operation performed on a key rather than the key itself into the script.
For example, we might want to insert a Hash160 of key **A**<sub>k</sub>.
We can use the following notation:

\[*Hash160*(**A**<sub>k</sub>)\]

### Encoding

TODO

## Examples

### 2-of-3 Multisig

The script template is defined by:

*Script*(**X**) = 2 \[**X**\] 3 OP\_CHECKMULTISIG

Letting **K**<sub>k</sub> = { **m**<sub>1</sub>/k, **m**<sub>2</sub>/k,
**m**<sub>3</sub>/k }, the *k*th script for this key group is denoted by
*Script*({**K**<sub>k</sub>}).

### 1-of-1 or 2-of-3

The script template is defined by:

*Script*(**A**, **B**) =  
        OP\_DUP \[**A**\] OP\_CHECKSIG  
        OP\_NOTIF  
                2 \[**B**\] 3 OP\_CHECKMULTISIGVERIFY  
        OP\_NOTIF  
        OP\_ENDIF  
        OP\_TRUE  
Let **M**<sub>k</sub> = **m**/k be a key of a superuser that can
authorize all transactions and {**K**<sub>k</sub>} be a key group of
three users that can only authorize transactions if at least two of them
agree.

The *k*th script is given by *Script*(**M**<sub>k</sub>,
{**K**<sub>k</sub>}).

### Timelocked Contract

The output is payable to Alice immediately if she knows the private key
for **A**<sub>k</sub>. Bob must know the private key for
**B**<sub>k</sub> and also wait for a timeout **t** before being able to
spend the output.

The script template is defined by:

*Script*(**A**, **B**, **T**) =  
        OP\_IF  
                OP\_DUP OP\_HASH160 \[*Hash160*(**A**)\] OP\_EQUALVERIFY
OP\_CHECKSIG  
        OP\_ELSE  
                \[**T**\] OP\_CHECKLOCKTIMEVERIFY OP\_DROP  
                OP\_DUP OP\_HASH160 \[*Hash160*(**B**)\] OP\_EQUALVERIFY
OP\_CHECKSIG  
        OP\_ENDIF

The *k*th script with timeout **t** is given by
*Script*(**A**<sub>k</sub>, **B**<sub>k</sub>, **t**).

## References

-   [BIP16 - Pay to Script Hash](bip-0016.mediawiki "wikilink")
-   [BIP32 - Hierarchical Deterministic
    Wallets](bip-0032.mediawiki "wikilink")
-   [BIP65 - OP\_CHECKLOCKTIMEVERIFY](bip-0065.mediawiki "wikilink")
-   [BIP112 - CHECKSEQUENCEVERIFY](bip-0112.mediawiki "wikilink")
-   [Lightning Network
    Whitepaper](https://lightning.network/lightning-network-paper.pdf "wikilink")

## Copyright

This document is placed in the public domain.
