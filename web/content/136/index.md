+++
title = "Bech32 Encoded Tx Position References"
date = 2017-07-09
weight = 136
in_search_index = true

[taxonomies]
authors = ["Велеслав", "Jonas Schnelli", "Daniel Pape"]
status = ["Draft"]

[extra]
bip = 136
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0136.mediawiki"
+++

``` 
  BIP: 136
  Layer: Applications
  Title: Bech32 Encoded Tx Position References
  Author: Велеслав <veleslav.bips@protonmail.com>
          Jonas Schnelli <dev@jonasschnelli.ch>
          Daniel Pape <dpape@dpape.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0136
  Status: Draft
  Type: Informational
  Created: 2017-07-09
  License: BSD-2-Clause
```

## Introduction

### Abstract

This document proposes a convenient human useable format, **"TxRef"**,
as a standard way to refer to a transaction position within the Bitcoin
Blockchain, and optionally a particular outpoint index within the
referred transaction. The primary purpose of this format is to allow
users to refer to a confirmed transaction (and optionally an outpoint
index within) in a standard, reliable, and concise way.

*Please note: Unlike TxID where there is strong cryptographic link
between the ID and the actual transaction, TxRef only provides a weak
link to a particular transaction. TxRef locates an offset within a
blockchain for a transaction, that may - or may not - point to an actual
transaction, which in fact may change with reorganisations. We recommend
that TxRef's should be not used for positions within the blockchain
having a maturity less than 100 blocks.*

### Copyright

This BIP is licensed under the 2-clause BSD license.

### Motivation

Since the first version of Bitcoin, TxID's (Transaction Identifiers)
have been a core part of the consensus protocol and have been routinely
used to identify individual transactions between users.

However, for many use-cases they have practical limitations:

  - TxIDs are expensive for full nodes to lookup (requiring either a
    linear scan of the blockchain, or an expensive TxID index).
  - TxIDs require third-party services for SPV wallets to lookup.
  - TxIDs are very long HEX encoded values (64 characters long).

For transactions that have been embedded in the blockchain, it is
possible to reference them not by their TxID, but by their location
within the blockchain itself. The encoding can be made friendly for
occasional human transcription. In this document, we propose a standard
for doing this.

### Examples

These examples are for Bitcoin Transactions.

  - Genesis Coinbase Transaction (Transaction \#0 of Block \#0):
    `tx1:rqqq-qqqq-qmhu-qhp`
  - Transaction \#2205 of Block \#466793: `tx1:rjk0-uqay-zsrw-hqe`

## Specification

A **confirmed transaction position reference**, or **TxRef**, is a
reference to a particular location within the blockchain, specified by
the block height and a transaction index within the block, and
optionally a outpoint index within the transaction.

*Please Note: All values in this specification are encoded in
little-endian format.*

### Transaction Position Reference Considerations

A TxRef may reference a location that doesn't exist because:

  - The specified block hasn't yet been mined. Or,
  - The transaction index is greater than the total number of
    transactions included within the specified block.
  - The optional outpoint index is greater than the total outpoints
    contained within the transaction.

Therefore, implementers must be careful not to display TxRef's to users
prematurely:

  - Applications MUST NOT display TxRef's for transactions with less
    than 6 confirmations.
  - Application MUST show a warning for TxRef's for transactions with
    less than 100 confirmations.
      - This warning SHOULD state that in the case of a large
        reorganisation, the TxRefs Displayed may point to a different
        transaction, or to no transaction at all.

### Encoding

TxRef uses standard Bech32\[1\] encoding as defined in
[BIP-173](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki)
and therefore consists of:

  - Human-readable Part, or "HRP", that provides namespacing. We have
    chosen to distinguish between Main and Test Networks:
      - For Any Mainnet Network: **"tx"**.
      - For Any Testnet Network: **"txtest"**.
      - Please see [SLIP-0173 : Registered human-readable parts for
        BIP-0173](https://github.com/satoshilabs/slips/blob/master/slip-0173.md)
        for a full list of HRP's including these two and others relating
        to other projects.
  - Separator: **"1"**.
  - Data Part.

Please note: other specifications, such as [the Decentralized
Identifiers spec](https://w3c-ccg.github.io/did-spec/), have implicitly
encoded the information contained within the HRP elsewhere. In this case
they may choose to not include the HRP as specified here.

To increase portability and readability additional separators SHOULD be
added:

  - A Colon\[2\] **":"** added after '1'.
  - Hyphens\[3\] **"-"** added after every 4 characters beyond the
    colon.

All non-bech32-alphabet characters after the bech32 code separator MUST
be ignored/removed when parsing (except for terminating
characters).\[4\]

|                     | Bit    | Character | Characters | Value                                                    |
| ------------------- | ------ | --------- | ---------- | -------------------------------------------------------- |
| Human Readable Part |        | 1 – 2     | 2          | Bitcoin Mainnet: "**tx**", Bitcoin Testnet: "**txtest**" |
| Separator           |        | 3         | 1          | "**1**"                                                  |
| Colon               |        | 4         | 1          | "**:**"                                                  |
| Data                | 0 – 19 | 5 – 8     | 4          |                                                          |
| Hyphen              |        | 9         | 1          | "**-**"                                                  |

Text Encoding of the TxRef

The Data - Hyphen pattern is repeated for the entire length of data, ( a
hyphen is inserted after every encoded 20 bits or 4 data characters).

### Data

Depending on if an optional transaction outpoint is included, there can
be 75 or 90 bits of data encoded in the string above. These bits are
defined in this manner:

|                   | **Bit** | **Bit(s)** | **Type**                             | **Values**                                                                                                                                      | **Notes**                  |
| ----------------- | ------- | ---------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| Magic Code        | 0 – 4   | 5          | Chain Namespacing Code               | **0x3** for Bitcoin Mainnet. **0x4** for Bitcoin Mainnet with Outpoint. **0x6** for Bitcoin Testnet. **0x7** for Bitcoin Testnet with Outpoint. |                            |
| Version           | 5       | 1          | For Future Use                       | Must be **0x0**                                                                                                                                 |                            |
| Block Height      | 6 – 29  | 24         | The Block Height of the Tx           | Block 0 (genesis) to block 16777215                                                                                                             | Until Year \~2328          |
| Transaction Index | 30 – 44 | 15         | The index of the Tx inside the block | Tx 0 (coinbase) to Tx position 32767                                                                                                            | Max Tx's in block is 16665 |

TxRef Binary Format for Bitcoin Mainnet and Bitcoin Testnet:

If the magic code is **0x4** or **0x7**, an optional outpoint is
included in the encoding:

|                | **Bit** | **Bit(s)** | **Type**                                | **Values**                            | **Notes** |
| -------------- | ------- | ---------- | --------------------------------------- | ------------------------------------- | --------- |
| Outpoint Index | 45 – 59 | 15         | The index of the Outpoint inside the Tx | Outpoint 0 to Outpoint Position 32767 |           |

Optional Outpoint Index Encoding:

We include the 30-bit checksum last:

|          | **Bit**            | **Bit(s)** | **Type**        | **Values** | **Notes** |
| -------- | ------------------ | ---------- | --------------- | ---------- | --------- |
| Checksum | 45 – 74 or 60 – 89 | 30         | Bech32 Checksum |            |           |

Bech32 Checksum Encoding:

#### Magic Notes:

The magic code provides namespacing between chains. 5-bit magic codes
are used for the Bitcoin Mainnet and the Bitcoin Testnet. (it may be
significantly longer for other projects/chains):

  - For Bitcoin Mainnet the magic code is: **0x3**, leading to an
    **"r"** character when encoded.
  - For Bitcoin Mainnet with Outpoint Encoded the magic code is:
    **0x4**, leading to an **"y"** character when encoded.
  - For Bitcoin Testnet the magic code is: **0x6**, leading to an
    **"x"** character when encoded.
  - For Bitcoin Testnet with Outpoint Encoded the magic code is:
    **0x7**, leading to an **"8"** character when encoded.

Codes **0x0**, **0x1**, **0x2**, **0x5**, are also reserved for future
use within the Bitcoin project.

*Any other chain MUST NOT start their magic code with any value between
0x0 and 0x7 inclusive.*

Other magic codes will be specified in SLIP-XXXX "TxRef for Non-Bitcoin
Chains and Networks".

### Compatibility

There are no known compatibility issues.

## Rationale

<references />

## Reference implementations

C Reference Implementation (supports magic codes 0x3 and 0x6):
<https://github.com/jonasschnelli/bitcoin_txref_code>

Go Reference Implementation (supports magic codes 0x3 and 0x6):
<https://github.com/kulpreet/txref>

C++ Reference Implementation (support magic codes 0x3, 0x4, 0x6, 0x7):
<https://github.com/dcdpr/btcr-DID-method/>

## Appendices

### Test Vectors

There are two sets of Test Vectors included here:

  - Bech32 Encoding Test Vectors. These are to test if a implementation
    accepts the encoding, with the correct human readable part, and
    separator.
  - Bitcoin TxRef Test Vectors. These test the full specification, in
    particular, correct values for block height and the transaction
    index.

#### Bech32 Encoding (for TxRef).

*Please Note: All test vectors are shown to help test if a string is
compliant or not. All real-life applications (such as for Bitcoin)
should comply with the Bitcoin Test Vectors listed Below.*

The following strings have a valid Human Readable Part and Bech32
Checksum.

  - `TX1A12UEL5L`
  - `tx1an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs`
  - `tx1abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw`
  - `tx11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j`

The following list gives invalid TxRef's and the reason for their
invalidity.

  - `bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kg3g4ty`: Invalid human-readable
    part
  - `tx1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5`: Invalid checksum

#### Bitcoin TxRef (mainnet and testnet)

The following list gives properly encoded Bitcoin mainnet TxRef's and
the values in hex. (block height, transaction index)

  - `tx1:rqqq-qqqq-qmhu-qhp`: `(0x0, 0x0)`
  - `tx1:rqqq-qqll-l8xh-jkg`: `(0x0, 0x7FFF)`
  - `tx1:r7ll-llqq-qghq-qr8`: `(0xFFFFFF, 0x0)`
  - `tx1:r7ll-llll-l5xt-jzw`: `(0xFFFFFF, 0x7FFF)`

The following list gives properly encoded Bitcoin testnet TxRef's and
the values in hex. (block height, transaction index)

  - `txtest1:xqqq-qqqq-qkla-64l`: `(0x0, 0x0)`
  - `txtest1:xqqq-qqll-l2wk-g5k`: `(0x0, 0x7FFF)`
  - `txtest1:x7ll-llqq-q9lp-6pe`: `(0xFFFFFF, 0x0)`
  - `txtest1:x7ll-llll-lew2-gqs`: `(0xFFFFFF, 0x7FFF)`

The following list gives valid (though strangely formatted) Bitcoin
TxRef's and the values in hex. (block height, transaction index)

  - `tx1:rjk0-uqay-zsrw-hqe`: `(0x71F69, 0x89D)`
  - `TX1RJK0UQAYZSRWHQE`: `(0x71F69, 0x89D)`
  - `TX1RJK0--UQaYZSRw----HQE`: `(0x71F69, 0x89D)`
  - `tx1 rjk0 uqay zsrw hqe`: `(0x71F69, 0x89D)`
  - `tx1!rjk0\uqay*zsrw^^hqe`: `(0x71F69, 0x89D)`

The following list gives invalid Bitcoin TxRef's and the reason for
their invalidity.

  - `tx1:t7ll-llll-ldup-3hh`: Magic 0xB instead of 0x3.
    `(0xFFFFFF, 0x7FFF)`
  - `tx1:rlll-llll-lfet-r2y`: Version 1 instead of 0.
    `(0xFFFFFF, 0x7FFF)`
  - `tx1:rjk0-u5ng-gghq-fkg7`: Valid Bech32, but 10x5bit packages
    instead of 8.
  - `tx1:rjk0-u5qd-s43z`: Valid Bech32, but 6x5bit packages instead of
    8.

#### Bitcoin TxRef with Outpoints (mainnet and testnet)

The following list gives properly encoded Bitcoin mainnet TxRef's with
Outpoints and the values in hex. (block height, transaction index, TXO
index)

  - `tx1:yqqq-qqqq-qqqq-ksvh-26`: `(0x0, 0x0, 0x0)`
  - `tx1:yqqq-qqll-lqqq-v0h2-2k`: `(0x0, 0x7FFF, 0x0)`
  - `tx1:y7ll-llqq-qqqq-a5zy-tc`: `(0xFFFFFF, 0x0, 0x0)`
  - `tx1:y7ll-llll-lqqq-8tee-t5`: `(0xFFFFFF, 0x7FFF, 0x0)`

<!-- end list -->

  - `tx1:yqqq-qqqq-qpqq-5j9q-nz`: `(0x0, 0x0, 0x1)`
  - `tx1:yqqq-qqll-lpqq-wd7a-nw`: `(0x0, 0x7FFF, 0x1)`
  - `tx1:y7ll-llqq-qpqq-lktn-jq`: `(0xFFFFFF, 0x0, 0x1)`
  - `tx1:y7ll-llll-lpqq-9fsw-jv`: `(0xFFFFFF, 0x7FFF, 0x1)`

<!-- end list -->

  - `tx1:yjk0-uqay-zrfq-g2cg-t8`: `(0x71F69, 0x89D, 0x123)`
  - `tx1:yjk0-uqay-zu4x-nk6u-pc`: `(0x71F69, 0x89D, 0x1ABC)`

The following list gives properly encoded Bitcoin testnet TxRef's with
Outpoints and the values in hex. (block height, transaction index, TXO
index)

  - `txtest1:8qqq-qqqq-qqqq-cgru-fa`: `(0x0, 0x0, 0x0)`
  - `txtest1:8qqq-qqll-lqqq-zhcp-f3`: `(0x0, 0x7FFF, 0x0)`
  - `txtest1:87ll-llqq-qqqq-nvd0-gl`: `(0xFFFFFF, 0x0, 0x0)`
  - `txtest1:87ll-llll-lqqq-fnkj-gn`: `(0xFFFFFF, 0x7FFF, 0x0)`

<!-- end list -->

  - `txtest1:8qqq-qqqq-qpqq-622t-s9`: `(0x0, 0x0, 0x1)`
  - `txtest1:8qqq-qqll-lpqq-q43k-sf`: `(0x0, 0x7FFF, 0x1)`
  - `txtest1:87ll-llqq-qpqq-3wyc-38`: `(0xFFFFFF, 0x0, 0x1)`
  - `txtest1:87ll-llll-lpqq-t3l9-3t`: `(0xFFFFFF, 0x7FFF, 0x1)`

<!-- end list -->

  - `txtest1:8jk0-uqay-zrfq-xjhr-gq`: `(0x71F69, 0x89D, 0x123)`
  - `txtest1:8jk0-uqay-zu4x-aw4h-zl`: `(0x71F69, 0x89D, 0x1ABC)`

### Bitcoin TxRef Payload Value Choice:

Some calculations showing why we chose these particular bit-length of
the block height and transaction index.

#### Block Height Value:

24-bit: between 0, and 0xFFFFFF (16,777,216 blocks).

  - There are \~52,500 blocks every year, leading to \~319 years of
    blocks addressable.
  - Therefore before year 2328 this specification should be extended.
    (We think that we have plenty of time).

#### Tx Position Value:

15-bit: between 0x0, and 0x7FFF. (32,768 transactions).

  - The *realistic* smallest Tx is 83 Bytes: Max 12047 tx in a block.
      - 4B version + 1B tx\_in count + 36B previous\_output + 1B script
        length + 0B signature script + 4B sequence + 1B tx\_out count +
        8B amount + 1B script length + 23B pubkey script + 4B lock\_time
        = 83B
  - The *extreme* smallest Tx is 60 Byte's: Max 16665 tx in a block.
      - 4B version + 1B tx\_in count + 36B previous\_output + 1B script
        length + 0B signature script + 4B sequence + 1B tx\_out count +
        8B amount + 1B script length + 0B pubkey script + 4B lock\_time
        = 60B

## Acknowledgements

Special Thanks to Pieter Wuille and Greg Maxwell for Bech32, a wonderful
user-facing data encoding.

1.  **Why use Bech32 Encoding for Confirmed Transaction References?**
    The error detection and correction properties of this encoding
    format make it very attractive. We expect that it will be reasonable
    for software to correct a maximum of two characters; however, we
    haven’t specified this yet.
2.  **Why add a colon here?** This allows it to conform better with W3C
    URN/URL standards.
3.  **Why hyphens within the TxRef?** As TxRef's are short, we expect
    that they will be quoted via voice or written by hand. The inclusion
    of hyphens every 4 characters breaks up the string and means people
    don't lose their place so easily.
4.  **Why strip all non-bech32-alphabet characters?** We do not wish to
    expect the users to keep their TxRef's in good unicode form
    (hyphens, colons, invisible spaces, random unicode characters, etc).
    We expect them to copy, paste, write by-hand, write in a mix of
    character sets, etc. Parsers should automatically correct for all
    sorts of these common errors.
