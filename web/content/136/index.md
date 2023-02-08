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

## Introduction

### Abstract

This document proposes a convenient, human usable encoding to refer to a
**confirmed transaction position** within the Bitcoin blockchain--known
as **"TxRef"**. The primary purpose of this encoding is to allow users
to refer to a confirmed transaction (and optionally, a particular
outpoint index within the transaction) in a standard, reliable, and
concise way.

*Please note: Unlike a transaction ID, **"TxID"**, where there is a
strong cryptographic link between the ID and the actual transaction, a
**TxRef** only provides a weak link to a particular transaction. A
**TxRef** locates an offset within a blockchain for a transaction, that
may - or may not - point to an actual transaction, which in fact may
change with reorganisations. We recommend that **TxRef**s should be not
used for positions within the blockchain having a maturity less than 100
blocks.*

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
document are to be interpreted as described in [RFC
2119](https://tools.ietf.org/html/rfc2119).

### Copyright

This BIP is licensed under the 2-clause BSD license.

### Motivation

Since the first version of Bitcoin, **TxID**s have been a core part of
the consensus protocol and are routinely used to identify individual
transactions between users.

However, for many use-cases they have practical limitations:

- **TxID**s are expensive for full nodes to lookup (requiring either a
  linear scan of the blockchain, or an expensive **TxID** index).
- **TxID**s require third-party services for SPV wallets to lookup.
- **TxID**s are 64 character HEX encoded values.

It is possible to reference transactions not only by their **TxID**, but
by their location within the blockchain itself. Rather than use the 64
character **TxID**, an encoding of the position coordinates can be made
friendly for occasional human transcription. In this document, we
propose a standard for doing this.

### Examples

| Block \# | Transaction \# | Outpoint \# | TxRef                      | TxID                                                             |
|----------|----------------|-------------|----------------------------|------------------------------------------------------------------|
| 0        | 0              | 0           | tx1:rqqq‑qqqq‑qwtv‑vjr     | 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b |
| 170      | 1              | 0           | tx1:r52q‑qqpq‑qpty‑cfg     | f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16 |
| 456789   | 1234           | 1           | tx1:y29u‑mqjx‑ppqq‑sfp2‑tt | 6fb8960f70667dc9666329728a19917937896fc476dfc54a3e802e887ecb4e82 |

## Specification

A **confirmed transaction position reference**, or **TxRef**, is a
reference to a particular location within the blockchain, specified by
the block height and a transaction index within the block, and
optionally, an outpoint index within the transaction.

*Please Note: All values in this specification are encoded in
little-endian format.*

### TxRef Considerations

It is possible for a **TxRef** to reference a transaction that doesn't
really exist because:

- The specified block hasn't yet been mined.
- The transaction index is greater than the total number of transactions
  included within the specified block.
- The optional outpoint index is greater than the total outpoints
  contained within the transaction.

Therefore, implementers must be careful not to display **TxRef**s to
users prematurely:

- Applications MUST NOT display **TxRef**s for transactions with less
  than 6 confirmations.
- Application MUST show a warning for **TxRef**s for transactions with
  less than 100 confirmations.
  - This warning SHOULD state that in the case of a large
    reorganisation, the **TxRef**s displayed may point to a different
    transaction, or to no transaction at all.

### TxRef Format

**TxRef** MUST use the **Bech32m**[^1] encoding as defined in
[BIP-0173](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki)
and later refined in
[BIP-0350](https://github.com/bitcoin/bips/blob/master/bip-0350.mediawiki).
The Bech32m encoding consists of:

#### Human-Readable Part

The **HRP** can be thought of as a label. We have chosen labels to
distinguish between Main, Test, and Regtest networks:

- Mainnet: **"tx"**.
- Testnet: **"txtest"**.
- Regtest: **"txrt"**.

#### Separator

The separator is the character **"1"**.

#### Data Part

The data part for a **TxRef** consists of the transaction's block
height, transaction index within the block, and optionally, an outpoint
index. Specific encoding details for the data are given below.

*Please note: other specifications, such as [the Decentralized
Identifiers spec](https://w3c-ccg.github.io/did-spec/), have implicitly
encoded the information contained within the HRP elsewhere. In this case
they may choose to not include the HRP as specified here.*

#### Readability

To increase portability and readability, additional separator characters
SHOULD be added to the **TxRef**:

- A Colon[^2] **":"** added after the separator character '1'.
- Hyphens[^3] **"-"** added after every 4 characters beyond the colon.

### Encoding

Encoding a **TxRef** requires 4 or 5 pieces of data: a magic code
denoting which network is being used; a version number (currently always
0); the block height of the block containing the transaction; the index
of the transaction within the block; and optionally, the index of the
outpoint within the transaction. Only a certain number of bits are
supported for each of these values, see the following table for details.

<table>
<thead>
<tr class="header">
<th></th>
<th><p>Description</p></th>
<th><p>Possible Data Type</p></th>
<th><p><strong># of Bits used</strong></p></th>
<th><p>Values</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Magic Code</p></td>
<td><p>Chain Namespacing Code</p></td>
<td><p>uint8</p></td>
<td><p>5</p></td>
<td><p><strong>3</strong>: Mainnet<br />
<strong>4</strong>: Mainnet with Outpoint<br />
<strong>6</strong>: Testnet<br />
<strong>7</strong>: Testnet with Outpoint<br />
<strong>0</strong>: Regtest<br />
<strong>1</strong>: Regtest with Outpoint</p></td>
</tr>
<tr class="even">
<td><p>Version</p></td>
<td><p>For Future Use</p></td>
<td><p>uint8</p></td>
<td><p>1</p></td>
<td><p>Must be <strong>0</strong></p></td>
</tr>
<tr class="odd">
<td><p>Block<br />
Height</p></td>
<td><p>The Block Height of the Tx</p></td>
<td><p>uint32</p></td>
<td><p>24</p></td>
<td><p>Block 0 to Block 16777215</p></td>
</tr>
<tr class="even">
<td><p>Transaction<br />
Index</p></td>
<td><p>The index of the Tx inside the block</p></td>
<td><p>uint16, uint32</p></td>
<td><p>15</p></td>
<td><p>Tx 0 to Tx 32767</p></td>
</tr>
<tr class="odd">
<td><p>Outpoint<br />
Index</p></td>
<td><p>The index of the Outpoint inside the Tx</p></td>
<td><p>uint16, uint32</p></td>
<td><p>15</p></td>
<td><p>Outpoint 0 to Outpoint 32767</p></td>
</tr>
</tbody>
</table>

#### Magic Notes

The magic code provides namespacing between chains:

- For Mainnet the magic code is: **0x3**, leading to an **"r"**
  character when encoded.
- For Mainnet with Outpoint Encoded the magic code is: **0x4**, leading
  to a **"y"** character when encoded.
- For Testnet the magic code is: **0x6**, leading to an **"x"**
  character when encoded.
- For Testnet with Outpoint Encoded the magic code is: **0x7**, leading
  to an **"8"** character when encoded.
- For Regtest the magic code is: **0x0**, leading to a **"q"** character
  when encoded.
- For Regtest with Outpoint Encoded the magic code is: **0x1**, leading
  to a **"p"** character when encoded.

#### Encoding Example

We want to encode a **TxRef** that refers to Transaction \#1234 of Block
\#456789 on the Mainnet chain. We use this data in preparation for the
Bech32 encoding algorithm:

<table>
<thead>
<tr class="header">
<th></th>
<th><p>Decimal<br />
Value</p></th>
<th><p>Binary<br />
Value</p></th>
<th><p><strong># of Bits<br />
used</strong></p></th>
<th><p>Bit Indexes and Values</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Magic<br />
Code</p></td>
<td><p>3</p></td>
<td><p>00000011</p></td>
<td><p>5</p></td>
<td><p>(mc04, mc03, mc02, mc01, mc00) = (0, 0, 0, 1, 1)</p></td>
</tr>
<tr class="even">
<td><p>Version</p></td>
<td><p>0</p></td>
<td><p>00000000</p></td>
<td><p>1</p></td>
<td><p>(v0) = (0)</p></td>
</tr>
<tr class="odd">
<td><p>Block<br />
Height</p></td>
<td><p>456789</p></td>
<td><p>00000110<br />
11111000<br />
01010101</p></td>
<td><p>24</p></td>
<td><p>(bh23, bh22, bh21, bh20, bh19, bh18, bh17, bh16) = (0, 0, 0, 0,
0, 1, 1, 0)<br />
(bh15, bh14, bh13, bh12, bh11, bh10, bh09, bh08) = (1, 1, 1, 1, 1, 0, 0,
0)<br />
(bh07, bh06, bh05, bh04, bh03, bh02, bh01, bh00) = (0, 1, 0, 1, 0, 1, 0,
1)</p></td>
</tr>
<tr class="even">
<td><p>Transaction<br />
Index</p></td>
<td><p>1234</p></td>
<td><p>00000100<br />
11010010</p></td>
<td><p>15</p></td>
<td><p>(ti14, ti13, ti12, ti11, ti10, ti09, ti08) = (0, 0, 0, 0, 1, 0,
0)<br />
(ti07, ti06, ti05, ti04, ti03, ti02, ti01, ti00) = (1, 1, 0, 1, 0, 0, 1,
0)</p></td>
</tr>
</tbody>
</table>

As shown in the last column, we take the necessary bits of each binary
value and copy them into nine unsigned chars illustrated in the next
table. We only set the lower five bits of each unsigned char as the
bech32 algorithm only uses those bits.

<table>
<thead>
<tr class="header">
<th></th>
<th></th>
<th><p>7</p></th>
<th><p>6</p></th>
<th><p>5</p></th>
<th><p>4</p></th>
<th><p>3</p></th>
<th><p>2</p></th>
<th><p>1</p></th>
<th><p>0</p></th>
<th></th>
<th><p>Decimal<br />
Value</p></th>
<th><p>Bech32<br />
Character</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[0]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>mc04</p></td>
<td><p>mc03</p></td>
<td><p>mc02</p></td>
<td><p>mc01</p></td>
<td><p>mc00</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td></td>
<td><p>3</p></td>
<td><p>r</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[1]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>bh03</p></td>
<td><p>bh02</p></td>
<td><p>bh01</p></td>
<td><p>bh00</p></td>
<td><p>v0</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td></td>
<td><p>10</p></td>
<td><p>2</p></td>
<td></td>
</tr>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[2]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>bh08</p></td>
<td><p>bh07</p></td>
<td><p>bh06</p></td>
<td><p>bh05</p></td>
<td><p>bh04</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td></td>
<td><p>5</p></td>
<td><p>9</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[3]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>bh13</p></td>
<td><p>bh12</p></td>
<td><p>bh11</p></td>
<td><p>bh10</p></td>
<td><p>bh09</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td></td>
<td><p>28</p></td>
<td><p>u</p></td>
<td></td>
</tr>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[4]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>bh18</p></td>
<td><p>bh17</p></td>
<td><p>bh16</p></td>
<td><p>bh15</p></td>
<td><p>bh14</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td></td>
<td><p>27</p></td>
<td><p>m</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[5]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>bh23</p></td>
<td><p>bh22</p></td>
<td><p>bh21</p></td>
<td><p>bh20</p></td>
<td><p>bh19</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td></td>
<td><p>0</p></td>
<td><p>q</p></td>
<td></td>
</tr>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[6]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>ti04</p></td>
<td><p>ti03</p></td>
<td><p>ti02</p></td>
<td><p>ti01</p></td>
<td><p>ti00</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td></td>
<td><p>18</p></td>
<td><p>j</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[7]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>ti09</p></td>
<td><p>ti08</p></td>
<td><p>ti07</p></td>
<td><p>ti06</p></td>
<td><p>ti05</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td></td>
<td><p>6</p></td>
<td><p>x</p></td>
<td></td>
</tr>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[8]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>ti14</p></td>
<td><p>ti13</p></td>
<td><p>ti12</p></td>
<td><p>ti11</p></td>
<td><p>ti10</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td></td>
<td><p>1</p></td>
<td><p>p</p></td>
<td></td>
</tr>
</tbody>
</table>

The Bech32 algorithm encodes the nine unsigned chars above and computes
a checksum of those chars and encodes that as well--this gives a six
character checksum (in this case, **utt3p0**) which is appended to the
final **TxRef**. The final **TxRef** given is:
**tx1:r29u-mqjx-putt-3p0** and is illustrated in the following table:

TxRef character indexes and descriptions

| Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  | 18  | 19  | 20  | 21  |
|-------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| Char: | t   | x   | 1   | :   | r   | 2   | 9   | u   | \-  | m   | q   | j   | x   | \-  | p   | u   | t   | t   | \-  | 3   | p   | 0   |

#### Outpoint Index

Some uses of **TxRef** may want to refer to a specific outpoint of the
transaction. In the previous example, since we did not specify the
outpoint index, the **TxRef** **tx1:r29u-mqjx-putt-3p0** implicitly
references the first (index 0) outpoint of the 1234th transaction in the
456789th block in the blockchain.

If instead, for example, we want to reference the second (index 1)
outpoint, we need to change the magic code from **3** to **4** and would
include the following in the data to be encoded:

<table>
<thead>
<tr class="header">
<th></th>
<th><p>Decimal<br />
Value</p></th>
<th><p>Binary<br />
Value</p></th>
<th><p><strong># of Bits<br />
used</strong></p></th>
<th><p>Bit Indexes and Values</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Magic<br />
Code</p></td>
<td><p>4</p></td>
<td><p>00000100</p></td>
<td><p>5</p></td>
<td><p>(mc04, mc03, mc02, mc01, mc00) = (0, 0, 1, 0, 0)</p></td>
</tr>
<tr class="even">
<td><p>Outpoint Index</p></td>
<td><p>1</p></td>
<td><p>00000000 00000001</p></td>
<td><p>15</p></td>
<td><p>(op14, op13, op12, op11, op10, op09, op08) = (0, 0, 0, 0, 0, 0,
0)<br />
(op07, op06, op05, op04, op03, op02, op01, op00) = (0, 0, 0, 0, 0, 0, 0,
1)</p></td>
</tr>
</tbody>
</table>

<table>
<thead>
<tr class="header">
<th></th>
<th></th>
<th><p>7</p></th>
<th><p>6</p></th>
<th><p>5</p></th>
<th><p>4</p></th>
<th><p>3</p></th>
<th><p>2</p></th>
<th><p>1</p></th>
<th><p>0</p></th>
<th></th>
<th><p>Decimal<br />
Value</p></th>
<th><p>Bech32<br />
Character</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[0]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>mc04</p></td>
<td><p>mc03</p></td>
<td><p>mc02</p></td>
<td><p>mc01</p></td>
<td><p>mc00</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td></td>
<td><p>4</p></td>
<td><p>y</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[9]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>op04</p></td>
<td><p>op03</p></td>
<td><p>op02</p></td>
<td><p>op01</p></td>
<td><p>op00</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>1</p></td>
<td></td>
<td><p>1</p></td>
<td><p>p</p></td>
<td></td>
</tr>
<tr class="odd">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>data[10]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>op09</p></td>
<td><p>op08</p></td>
<td><p>op07</p></td>
<td><p>op06</p></td>
<td><p>op05</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td></td>
<td><p>0</p></td>
<td><p>q</p></td>
<td></td>
</tr>
<tr class="even">
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><p>data[11]</p></td>
<td><p>Index</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>na</p></td>
<td><p>op14</p></td>
<td><p>op13</p></td>
<td><p>op12</p></td>
<td><p>op11</p></td>
<td><p>op10</p></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr class="even">
<td><p>Value</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td><p>0</p></td>
<td></td>
<td><p>0</p></td>
<td><p>q</p></td>
<td></td>
</tr>
</tbody>
</table>

After Bech32 encoding all twelve unsigned chars above, we get the
checksum: **sfp2tt**. The final **TxRef** given is:
**tx1:y29u-mqjx-ppqq-sfp2-tt** and is illustrated in the following
table:

TxRef character indexes and descriptions

| Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  |
|-------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| Char: | t   | x   | 1   | :   | y   | 2   | 9   | u   | \-  | m   | q   | j   | x   | \-  | p   | p   | q   | q   | \-  | s   | f   | p   | 2   | \-  | t   | t   |

### Decoding

The Bech32 spec defines 32 valid characters as its "alphabet". All
non-Bech32-alphabet characters present in a **TxRef** after the Bech32
separator character MUST be ignored/removed when parsing (except for
terminating characters). We do not wish to expect the users to keep
their **TxRef**s in good form and **TxRef**s may contains hyphens,
colons, invisible spaces, uppercase or random characters. We expect
users to copy, paste, write by-hand, write in a mix of character sets,
etc. Parsers SHOULD attempt to correct for these and other common
errors, reporting to the user any **TxRef**s that violate a proper
Bech32 encoding.

As of early 2021, **TxRef** has been in limited use for a couple of
years and it is possible that there are some **TxRef**s in use which
were created with the original specification of Bech32 before the
Bech32m refinement was codified. Due to this possibility, a **TxRef**
parser SHOULD be able to decode both Bech32m and Bech32 encoded
**TxRef**s. In such a case, a **TxRef** parser SHOULD display or somehow
notify the user that they are using an obsolete **TxRef** and that they
should upgrade it to the Bech32m version. Additionally, the parser MAY
also display the Bech32m version.

## Rationale

<references />

## Reference implementations

C Reference Implementation (supports magic codes 0x3 and 0x6):
<https://github.com/jonasschnelli/bitcoin_txref_code>

Go Reference Implementation (supports magic codes 0x3 and 0x6):
<https://github.com/kulpreet/txref>

C++ Reference Implementation (supports magic codes 0x3, 0x4, 0x6, 0x7,
0x0 and 0x1): <https://github.com/dcdpr/libtxref/>

Java Reference Implementation (supports magic codes 0x3, 0x4, 0x6, 0x7,
0x0 and 0x1): <https://github.com/dcdpr/libtxref-java/>

## Appendices

### Test Examples

The following examples show values for various combinations on mainnet
and testnet; encoding block height, transaction index, and an optional
output index.

#### TxRef

The following list gives properly encoded mainnet **TxRef**s and the
decoded hex values (block height, transaction index)

- `tx1:rqqq-qqqq-qwtv-vjr`: `(0x0, 0x0)`
- `tx1:rqqq-qqll-lj68-7n2`: `(0x0, 0x7FFF)`
- `tx1:r7ll-llqq-qats-vx9`: `(0xFFFFFF, 0x0)`
- `tx1:r7ll-llll-lp6m-78v`: `(0xFFFFFF, 0x7FFF)`

The following list gives properly encoded testnet **TxRef**s and the
decoded hex values (block height, transaction index)

- `txtest1:xqqq-qqqq-qrrd-ksa`: `(0x0, 0x0)`
- `txtest1:xqqq-qqll-lljx-y35`: `(0x0, 0x7FFF)`
- `txtest1:x7ll-llqq-qsr3-kym`: `(0xFFFFFF, 0x0)`
- `txtest1:x7ll-llll-lvj6-y9j`: `(0xFFFFFF, 0x7FFF)`

The following list gives valid (sometimes strangely formatted)
**TxRef**s and the decoded values (block height, transaction index)\*

- `tx1:r29u-mqjx-putt-3p0`: `(456789, 1234)`
- `TX1R29UMQJXPUTT3P0`: `(456789, 1234)`
- `tx1 r29u mqjx putt 3p0`: `(456789, 1234)`
- `tx1!r29u/mqj*x-putt^^3p0`: `(456789, 1234)`

The following list gives invalid **TxRef**s and the reason for their
invalidity.

- `tx1:t7ll-llll-lcq3-aj4`: Magic 0xB instead of 0x3.
- `tx1:rlll-llll-lu9m-00x`: Version 1 instead of 0.
- `tx1:r7ll-llll-lqfu-gss2`: Valid Bech32, but ten 5 bit unsigned chars
  instead of nine.
- `tx1:r7ll-llll-rt5h-wz`: Valid Bech32, but eight 5 bit unsigned chars
  instead of nine.
- `tx1:r7ll-LLLL-lp6m-78v`: Invalid Bech32 due to mixed case. Would
  decode correctly otherwise.

#### TxRef with Outpoints

The following list gives properly encoded mainnet **TxRef**s with
Outpoints and the decoded values (block height, transaction index,
outpoint index)

- `tx1:yqqq-qqqq-qqqq-rvum-0c`: `(0x0, 0x0, 0x0)`
- `tx1:yqqq-qqll-lqqq-en8x-05`: `(0x0, 0x7FFF, 0x0)`
- `tx1:y7ll-llqq-qqqq-ggjg-w6`: `(0xFFFFFF, 0x0, 0x0)`
- `tx1:y7ll-llll-lqqq-jhf4-wk`: `(0xFFFFFF, 0x7FFF, 0x0)`

<!-- -->

- `tx1:yqqq-qqqq-qpqq-pw4v-kq`: `(0x0, 0x0, 0x1)`
- `tx1:yqqq-qqll-lpqq-m3w3-kv`: `(0x0, 0x7FFF, 0x1)`
- `tx1:y7ll-llqq-qpqq-22ml-hz`: `(0xFFFFFF, 0x0, 0x1)`
- `tx1:y7ll-llll-lpqq-s4qz-hw`: `(0xFFFFFF, 0x7FFF, 0x1)`

<!-- -->

- `tx1:y29u-mqjx-ppqq-sfp2-tt`: `(456789, 1234, 1)`

The following list gives properly encoded testnet **TxRef**s with
Outpoints and the decoded values (block height, transaction index,
outpoint index)

- `txtest1:8qqq-qqqq-qqqq-d5ns-vl`: `(0x0, 0x0, 0x0)`
- `txtest1:8qqq-qqll-lqqq-htgd-vn`: `(0x0, 0x7FFF, 0x0)`
- `txtest1:87ll-llqq-qqqq-xsar-da`: `(0xFFFFFF, 0x0, 0x0)`
- `txtest1:87ll-llll-lqqq-u0x7-d3`: `(0xFFFFFF, 0x7FFF, 0x0)`

<!-- -->

- `txtest1:8qqq-qqqq-qpqq-0k68-48`: `(0x0, 0x0, 0x1)`
- `txtest1:8qqq-qqll-lpqq-4fp6-4t`: `(0x0, 0x7FFF, 0x1)`
- `txtest1:87ll-llqq-qpqq-yj55-59`: `(0xFFFFFF, 0x0, 0x1)`
- `txtest1:87ll-llll-lpqq-7d0f-5f`: `(0xFFFFFF, 0x7FFF, 0x1)`

<!-- -->

- `txtest1:829u-mqjx-ppqq-73wp-gv`: `(456789, 1234, 1)`

### TxRef Payload Value Choices:

Some calculations showing why we chose these particular bit-length of
the block height and transaction index.

#### Block Height Value:

24 bits: value can be between 0, and 0xFFFFFF (16777216 blocks).

- In early April, 2021, there have been 677700 blocks
- There are roughly (365 days \* 24 hours \* 6 blocks / hour) = 52560
  blocks every year, implying about (16777216 - 677700) / 52560 = 306
  more years of addressable blocks.
- Some time before year 2327 this specification should be extended.

#### Tx Position Value:

15 bits: value can be between 0x0, and 0x7FFF (32768 transactions).

- The *realistic* smallest Tx is 83 Bytes for maximum 12047 tx in a
  block.
  - 4B version + 1B tx_in count + 36B previous_output + 1B script
    length + 0B signature script + 4B sequence + 1B tx_out count + 8B
    amount + 1B script length + 23B pubkey script + 4B lock_time = 83B
- The *extreme* smallest Tx is 60 Bytes for maximum 16665 tx in a block.
  - 4B version + 1B tx_in count + 36B previous_output + 1B script
    length + 0B signature script + 4B sequence + 1B tx_out count + 8B
    amount + 1B script length + 0B pubkey script + 4B lock_time = 60B

## Acknowledgements

Special Thanks to Pieter Wuille and Greg Maxwell for Bech32, a wonderful
user-facing data encoding.

[^1]: **Why use Bech32 Encoding for Confirmed Transaction References?**
    The error detection and correction properties of this encoding
    format make it very attractive. We expect that it will be reasonable
    for software to correct a maximum of two characters; however, we
    haven’t specified this yet.

[^2]: **Why add a colon here?** This allows it to conform better with
    W3C URN/URL standards.

[^3]: **Why hyphens within the TxRef?** As **TxRef**s are short, we
    expect that they will be quoted via voice or written by hand. The
    inclusion of hyphens every 4 characters breaks up the string and
    means people don't lose their place so easily.
