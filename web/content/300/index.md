+++
title = "Hashrate Escrows (Consensus layer)"
date = 2017-08-14
weight = 300
in_search_index = true

[taxonomies]
authors = ["Paul Sztorc", "CryptAxe"]
status = ["Draft"]

[extra]
bip = 300
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0300.mediawiki"
+++

``` 
  BIP: 300
  Layer: Consensus (soft fork)
  Title: Hashrate Escrows (Consensus layer)
  Author: Paul Sztorc <truthcoin@gmail.com>
          CryptAxe <cryptaxe@gmail.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0300
  Status: Draft
  Type: Standards Track
  Created: 2017-08-14
  License: BSD-2-Clause
  Post-History: https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2017-May/014364.html
```

## Abstract

In Bip300, txns are not signed via cryptographic key. Instead, they are
"signed" by the accumulation of hashpower over time.

Bip300 emphasizes slow, transparent, auditable transactions which are
easy for honest users to get right and very hard for dishonest users to
abuse. The chief design goal for Bip300 is *partitioning* -- users may
safely ignore Bip300 txns if they want to (or Bip300 entirely).

See [this site](http://www.drivechain.info/) for more information.

## Motivation

As Reid Hoffman [wrote
in 2014](https://blockstream.com/2015/01/13/en-reid-hoffman-on-the-future-of-the-bitcoin-ecosystem/):
"Sidechains allow developers to add features and functionality to the
Bitcoin universe without actually modifying the Bitcoin Core
code...Consequently, innovation can occur faster, in more flexible and
distributed ways, without losing the synergies of a common platform with
a single currency."

Coins such as Namecoin, Monero, ZCash, and Sia, offer features that
Bitcoiners cannot access -- not without selling their BTC to invest in a
rival monetary unit. According to
[coinmarketcap.com](https://coinmarketcap.com/charts/#dominance-percentage),
there is now more value \*outside\* the BTC protocol than within it.
According to [cryptofees.info](https://cryptofees.info/), 10x more txn
fees are paid outside the BTC protocol, than within it.

Software improvements to Bitcoin rely on developer consensus -- BTC will
pass on a good idea if it is even slightly controversial. Development is
slow: we are now averaging one major feature every 5 years.

Sidechains allow for competitive "benevolent dictators" to create a new
sidechain at any time. These dictators are accountable only to their
users, and (crucially) they are protected from rival dictators. Users
can move their BTC among these different pieces of software, as \*they\*
see fit.

BTC can copy every useful technology, as soon as it is invented;
scamcoins lose their justification and become obsolete; and the
community can be pro-creativity, knowing that Layer1 is protected from
harmful changes.

## Specification

Bip300 allows for six new blockchain messages:

  - M1. "Propose New Sidechain"
  - M2. "ACK Proposal"
  - M3. "Propose Bundle"
  - M4. "ACK Bundle"
  - M5. Deposit -- a transfer of BTC from-main-to-side
  - M6. Withdrawal -- a transfer of BTC from-side-to-main

Nodes organize those messages into two caches:

  - D1. "Escrow\_DB" -- tracks the 256 Hashrate Escrows (Escrows slots
    that a sidechain can live in).
  - D2. "Withdrawal\_DB" -- tracks the withdrawal-Bundles (coins leaving
    a Sidechain).

We will cover:

1.  Adding Sidechains (D1, M1, M2)
2.  Approving Withdrawals (D2, M3, M4)
3.  Depositing and Withdrawing (M5, M6)

### Adding Sidechains (D1, M1, M2)

#### D1 -- "Escrow\_DB"

The table below enumerates the new database fields, their size in bytes,
and their purpose. A sidechain designer is free to choose any value for
these.

<table>
<thead>
<tr class="header">
<th><p>Field No.</p></th>
<th><p>Label</p></th>
<th><p>Type</p></th>
<th><p>Description / Purpose</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>1</p></td>
<td><p>Escrow Number</p></td>
<td><p>uint8_t</p></td>
<td><p>The escrow's ID number. Used to uniquely refer to each sidechain.</p></td>
</tr>
<tr class="even">
<td><p>2</p></td>
<td><p>Version</p></td>
<td><p>int32_t</p></td>
<td><p>Version number.</p></td>
</tr>
<tr class="odd">
<td><p>3</p></td>
<td><p>String KeyID</p></td>
<td><p>string</p></td>
<td><p>Used to derive all sidechain deposit addresses.</p></td>
</tr>
<tr class="even">
<td><p>4<br />
</p></td>
<td><p>Sidechain Private Key</p></td>
<td><p>string</p></td>
<td><p>The private key of the sidechain deposit script.</p></td>
</tr>
<tr class="odd">
<td><p>5<br />
</p></td>
<td><p>ScriptPubKey</p></td>
<td><p>CScript</p></td>
<td><p>Where the sidechain coins go. This always stays the same, even though the CTIP (UTXO) containing the coins is always changing.</p></td>
</tr>
<tr class="even">
<td><p>6</p></td>
<td><p>Sidechain Name</p></td>
<td><p>string</p></td>
<td><p>A human-readable name of the sidechain.</p></td>
</tr>
<tr class="odd">
<td><p>7</p></td>
<td><p>Sidechain Description</p></td>
<td><p>string</p></td>
<td><p>A human-readable name description of the sidechain.</p></td>
</tr>
<tr class="even">
<td><p>8</p></td>
<td><p>Hash1 - tarball hash</p></td>
<td><p>uint256</p></td>
<td><p>Intended as the sha256 hash of the tar.gz of the canonical sidechain software. (This is not enforced anywhere by Bip300, and is for human purposes only.)</p></td>
</tr>
<tr class="odd">
<td><p>9</p></td>
<td><p>Hash2 - git commit hash</p></td>
<td><p>uint160</p></td>
<td><p>Intended as the git commit hash of the canonical sidechain node software. (This is not enforced anywhere by Bip300, and is for human purposes only.)</p></td>
</tr>
<tr class="even">
<td><p>10</p></td>
<td><p>Active</p></td>
<td><p>bool</p></td>
<td><p>Does this sidechain slot contain an active sidechain?<br />
</p></td>
</tr>
<tr class="odd">
<td><p>11</p></td>
<td><p>"CTIP" -- Part 1 "TxID"</p></td>
<td><p>uint256</p></td>
<td><p>The CTIP, or "Critical (TxID, Index) Pair" is a variable for keeping track of where the sidechain's money is (ie, which member of the UTXO set).</p></td>
</tr>
<tr class="even">
<td><p>12</p></td>
<td><p>"CTIP" -- Part 2 "Index"</p></td>
<td><p>int32_t</p></td>
<td><p>Of the CTIP, the second element of the pair: the Index. See #11 above.</p></td>
</tr>
</tbody>
</table>

D1 is updated via M1 and M2.

#### M1 -- "Propose New Sidechain"

Examples:

<img src="bip-0300/m1-gui.jpg?raw=true" align="middle"></img>

<img src="bip-0300/m1-cli.png?raw=true" align="middle"></img>

M1 is a coinbase OP Return output containing the following:

`   1-byte - OP_RETURN (0x6a)`  
`   4-byte - Message header (0xD5E0C4AF)`  
`   N-byte - The serialization of the sidechain.`  
`     1-byte nSidechain`  
`     4-byte nVersion`  
`     x-byte strKeyID`  
`     x-byte strPrivKey`  
`     x-byte scriptPubKey`  
`     x-byte title`  
`     x-byte description`  
`     32-byte hashID1`  
`     20-byte hashID2`

M1 is used in conjunction with M2.

#### M2 -- "ACK Sidechain Proposal"

`   1-byte - OP_RETURN (0x6a)`  
`   4-byte - Message header (0xD6E1C5BF)`  
`   32-byte - sha256D hash of sidechain's serialization`

#### M1/M2 Validation Rules

1.  Any miner can propose a new sidechain at any time. This procedure
    resembles BIP 9 soft fork activation: the network must see a
    properly-formatted M1, followed by "acknowledgment" of the sidechain
    in 90% of the following 2016 blocks.
2.  It is possible to "overwrite" a sidechain. This requires more ACKs
    -- 50% of the following 26300 blocks must contain an M2. The
    possibility of overwrite, does not change the security assumptions
    (because we already assume that users perform extra-protocolic
    validation at a rate of 1 bit per 26300 blocks).

### Approving Withdrawals (D2, M3, M4)

Withdrawals in Bip300 (ie, "M6"), are very significant. So, we will
first discuss how these are approved/rejected -- a process involving M3,
M4, and D2.

#### What are Bundles?

All Bip300 withdrawals take the form of “Bundles” (formerly known as
“WT^s”) -- named because they "bundle up" many individual
withdrawal-requests into one single rare layer1 transaction.

This bundle either pays all of the withdrawals out, or else it fails
(and pays nothing out). Bip300 / layer 1 does not assemble Bundles (the
sidechain developer does this in a manner of their choosing).

Bundles are identified by a 32-byte hash, which aspires to be the TxID
of M6. Unfortunately, the Bundle-hash and M6-TxID cannot match exactly,
since the first input to M6 is a CTIP which is constantly changing. So,
we must accomplish a task, which is conceptually similar to AnyPrevOut
(BIP 118). We define a "blinded TxID" as a way of hashing a txn, in
which some bytes are first overwritten with zeros. In our case, these
bytes are the first input and the first output.

D2 controls Bundles, and is driven by M3, M4, M5, and M6.

#### D2 -- "Withdrawal\_DB"

| Field No. | Label                  | Type      | Description / Purpose                                                                                                                                                                                  |
| --------- | ---------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1         | Sidechain Number       | uint8\_t  | Links the withdrawal-request to a specific hashrate escrow.                                                                                                                                            |
| 2         | Bundle Hash            | uint256   | A withdrawal attempt. Specifically, it is a "blinded transaction id" (ie, the double-Sha256 of a txn that has had two fields zeroed out, see M6) of a txn which could withdraw funds from a sidechain. |
| 3         | ACKs (Work Score)      | uint16\_t | The current ACK-counter, which is the total number of ACKs (the PoW that has been used to validate the Bundle).                                                                                        |
| 4         | Blocks Remaining (Age) | uint16\_t | The number of blocks which this Bundle has remaining to accumulate ACKs                                                                                                                                |

A hash of D2 exists in each coinbase txn, and has
consensus-significance.

#### D2 Validation Rules

1.  The D2 hash commitment must be in each block (unless D2 is blank).
2.  The Bundles must be listed in a canonical order (so that the hashes
    match).
3.  From one block to the next, "Age" fields must increase by exactly 1
    (ie, Blocks Remaining decreases by 1).
4.  Bundles are stored in D2 until they fail (which occurs at "Age" =
    "MaxAge"), or they succeed (Bundle is paid out).
5.  From one block to the next, the value in the ACKs field
    (ACK-counter) can increase or decrease by a maximum of 1 (see
    below).

If a Bundle succeeds (in D2), it "becomes" an M6 message and is included
in a block.

So, first: how do we add a Bundle to D2?

#### M3 -- "Propose Bundle"

Nodes will add an entry to D2 if there is a coinbase output with the
following:

`   1-byte - OP_RETURN (0x6a)`  
`   4-byte - Commitment header (0xD45AA943)`  
`   32-byte - The Bundle hash, to populate a new D2 entry`

#### M3 Validation Rules

1.  If the network detects a properly-formatted M3, it must add an entry
    to D2 in the very next block. The starting Blocks Remaining value is
    26,299. The starting ACKs count is 1.
2.  Each block can only contain one M3 per sidechain.

Once a Bundle is in D2, how can we give it enough ACKs to make it valid?

#### M4 -- "ACK Withdrawal"

From one block to the next, "ACKs" can only change as follows:

  - The ACK-counter of any Bundle can only change by (-1,0,+1).
  - Within a sidechain-group, upvoting one Bundle ("+1") requires you to
    downvote all other Bundles in that group. However, the minimum
    ACK-counter is zero.
  - While only one Bundle can be upvoted at once; the whole group can
    all be unchanged at once ("abstain"), and they can all be downvoted
    at once ("alarm").

M4 does not need to be explicitly transmitted. It can simply be inferred
from the new state of D2. M4 can therefore be improved over time,
without affecting consensus.

Nonetheless, one option for explicit transmission of M4 is [in our
code](https://github.com/drivechain-project/mainchain/blob/8901d469975752d799b6a7a61d4e00a9a124028f/src/validation.cpp#L3735-L3790).

Often, M4 does not need to be transmitted at all. If there are n
Sidechains and m Withdrawals-per-sidechain, then there are (m+2)^n total
candidates for the next D2. So, when m and n are low, all of the
possible D2s can be trivially computed in advance.

Miners can impose a "soft limit" on m, blocking new withdrawal-attempts
until previous ones expire. Even if they fail to do this, a a worst-case
scenario of n=200 and m=1,000, honest nodes can communicate the M4 with
\~25 KB per block \[4+1+1+(200\\\*(1000+1+1)/8)\].

Finally, we give Deposits and Withdrawals.

### Deposits and Withdrawals (M5, M6)

Both M5 and M6 are regular Bitcoin txns. They are identified, as
Deposits/Withdrawals, when they select one of the special CTIP UTXOs as
one of their inputs (see D1).

All of a sidechain’s coins, are stored in one UTXO.
(Deposits/Withdrawals never cause UTXO bloat.) So, each
Deposit/Withdrawal must select a CTIP, and generate a new CTIP (this is
tracked in D1, above).

If the from-CTIP-to-CTIP quantity of coins goes **up**, (ie, if the user
is adding coins), then the txn is treated as a Deposit (M5). Else it is
treated as a Withdrawal (M6). See
[here](https://github.com/drivechain-project/mainchain/blob/8901d469975752d799b6a7a61d4e00a9a124028f/src/validation.cpp#L668-L781).

#### M5. "Make a Deposit" -- a transfer of BTC from-main-to-side

As far as mainchain consensus is concerned, all deposits to a sidechain
are always valid.

#### M6. "Execute Withdrawal" -- a transfer of BTC from-side-to-main

We come, finally, to the critical matter: where users can take their
money \*out\* of the sidechain.

In each block, a Bundle in D2 is considered "approved" if its
"ACK-counter" value meets the threshold (13,150).

The Bundle must meet all these criteria:

1.  "Be ACKed" -- The "blinded TxID" of this txn must be a member of the
    "approved candidate" set in the D2 of this block.
2.  "Return Change to Account" -- TxOut0 must pay coins back to the
    sidechain's CTIP.
3.  "Return \*all\* Change to Account" -- Sum of inputs must equal the
    sum of outputs. No traditional tx fee is possible.

## Backward compatibility

As a soft fork, older software will continue to operate without
modification. Non-upgraded nodes will see a number of phenomena that
they don't understand -- coinbase txns with non-txn data, value
accumulating in anyone-can-spend UTXOs for months at a time, and then
random amounts leaving the UTXO in single, infrequent bursts. However,
these phenomena don't affect them, or the validity of the money that
they receive.

( As a nice bonus, note that the sidechains themselves inherit a
resistance to hard forks. The only way to guarantee that a sidechain's
Bundles will continue to match identically, is to upgrade sidechains via
soft forks of themselves. )

## Deployment

This BIP will be deployed by "version bits" BIP9 with the name
"hrescrow" and using bit 4.

    // Deployment of Drivechains (BIPX, BIPY)
    consensus.vDeployments[Consensus::DEPLOYMENT_DRIVECHAINS].bit = 4;
    consensus.vDeployments[Consensus::DEPLOYMENT_DRIVECHAINS].nStartTime = 1642276800; // January 15th, 2022.
    consensus.vDeployments[Consensus::DEPLOYMENT_DRIVECHAINS].nTimeout = 1673812800; // January 15th, 2023.

## Reference Implementation

See: <https://github.com/DriveNetTESTDRIVE/DriveNet>

Also, for interest, see an example sidechain here:
<https://github.com/drivechain-project/bitcoin/tree/sidechainBMM>

## References

See <http://www.drivechain.info/literature/index.html>

## Credits

Thanks to everyone who contributed to the discussion, especially:
ZmnSCPxj, Adam Back, Peter Todd, Dan Anderson, Sergio Demian Lerner,
Chris Stewart, Matt Corallo, Sjors Provoost, Tier Nolan, Erik Aronesty,
Jason Dreyzehner, Joe Miyamoto, Ben Goldhaber.

## Copyright

This BIP is licensed under the BSD 2-clause license.
