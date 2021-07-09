+++
title = "SIGHASH_ANYPREVOUT for Taproot Scripts"
date = 2017-02-28
weight = 118
in_search_index = true

[taxonomies]
authors = ["Christian Decker", "Anthony Towns"]
status = ["Draft"]

[extra]
bip = 118
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0118.mediawiki"
+++

``` 
  BIP: 118
  Layer: Consensus (soft fork)
  Title: SIGHASH_ANYPREVOUT for Taproot Scripts
  Author: Christian Decker <decker.christian@gmail.com>
          Anthony Towns <aj@erisian.com.au>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0118
  Status: Draft
  Type: Standards Track
  Created: 2017-02-28
  License: BSD-3-Clause
  Requires: 340, 341, 342
```

## Introduction

### Abstract

This BIP describes a new type of public key for tapscript ([BIP
342](bip-0342.mediawiki "wikilink")) transactions. It allows signatures
for these public keys to not commit to the exact UTXO being spent. This
enables dynamic binding of transactions to different UTXOs, provided
they have compatible scripts.

### Copyright

This document is licensed under the 3-clause BSD license.

### Motivation

Off-chain protocols make use of transactions that are not yet broadcast
to the Bitcoin network in order to renegotiate the final state that
should be settled on-chain. In a number of cases it is desirable to
respond to a given transaction being seen on-chain with a predetermined
reaction in the form of another transaction. Often the same reaction is
desired for a variety of different transactions that may be seen
on-chain, but because the input signatures in the response transaction
commit to the exact transaction that is being reacted to, this means a
new signature must be created for every possible transaction one wishes
to be able to react to.

This proposal introduces a new public key type\[1\] that modifies the
behavior of the transaction digest algorithm used in the signature
creation and verification, by excluding the commitment to the previous
output (and, optionally, the witness script\[2\] and value \[3\]).
Removing this commitment allows dynamic rebinding of a signed
transaction to another previous output that requires authorisation by
the same key.

The dynamic rebinding is opt-in due to using a separate public key type,
and the breadth of transactions the signature can be rebound to can be
further restricted by using different keys, committing to the script
being spent in the signature, using different amounts between UTXOs,
using different nSequence values in the spending transaction, or using
the codeseparator opcode to commit to the position in the script.

## Specification

This BIP modifies the behaviour of the [BIP
342](bip-0342.mediawiki "wikilink") signature opcodes\[4\] (`CHECKSIG`,
`CHECKSIGVERIFY`, and `CHECKSIGADD`) for public keys that have a length
of 33 bytes and a first byte of `0x01` or the public key which is
precisely the single byte vector `0x01`\[5\]. These keys are termed
**BIP 118 public keys**.

#### Rules for signature opcodes

The [BIP 342](bip-0342.mediawiki "wikilink") rules for signature opcodes
are modified by removing keys with the first byte `0x01` and length of
either 1-byte or 33-bytes from the list of unknown public key types, and
adding the following rule prior to the handling of unknown public key
types:

  - If the public key is the single byte `0x01`, or if the public key is
    33 bytes and the first byte of the public key is `0x01`, it is
    considered to be a BIP 118 public key:
      - If the signature is not the empty vector, the signature is
        validated according to the [BIP
        341](bip-0341.mediawiki "wikilink") signing validation rules
        with the public key, allowable `hash_type` values, and
        transaction digest modified as defined below.

#### Public key

To convert the 1-byte BIP 118 public key for use with [BIP
340](bip-0340.mediawiki "wikilink"), use the 32-byte taproot internal
key, `p`, as defined in [BIP 341](bip-0341.mediawiki "wikilink").

To convert a 33-byte BIP 118 public key for use with [BIP
340](bip-0340.mediawiki "wikilink"), remove the `0x01` prefix and use
the remaining 32 bytes.

#### Signature message

The function *SigMsg118(hash\_type, ext\_flag)* computes the message
being signed as a byte array, analogously to *SigMsg(hash\_type,
ext\_flag)* defined in [BIP 341](bip-0341.mediawiki "wikilink"),
*SigExt118(hash\_type,key\_version)* computes the extension, similarly
to [BIP 342](bip-0342.mediawiki "wikilink").

The parameter *hash\_type* is an 8-bit unsigned value, reusing values
defined in [BIP 341](bip-0341.mediawiki "wikilink"), with the addition
that the values `0x41`, `0x42`, `0x43`, `0xc1`, `0xc2`, and `0xc3` are
also valid for BIP 118 public keys.

We define the following constants using bits 6 and 7 of `hash_type`:

  - `SIGHASH_ANYPREVOUT = 0x40`
  - `SIGHASH_ANYPREVOUTANYSCRIPT = 0xc0`

As per [BIP 341](bip-0341.mediawiki "wikilink"), the parameter
*ext\_flag* is an integer in the range 0-127, used for indicating that
extensions are added at the end of the message. The parameter
*key\_version* is an 8-bit unsigned value (an integer in the range
0-255) used for committing to the public key version.

The following restrictions apply and cause validation failure if
violated:

  - Using any undefined *hash\_type* (not *0x00*, *0x01*, *0x02*,
    *0x03*, *0x41*, *0x42*, *0x43*, *0x81*, *0x82*, *0x83*, *0xc1*,
    *0xc2*, or *0xc3*).
  - Using `SIGHASH_SINGLE` without a "corresponding output" (an output
    with the same index as the input being verified).

If these restrictions aren't violated, *SigMsg118(hash\_type,ext\_flag)*
evaluates to the concatenation of the following data, in order (with
byte size of each item listed in parentheses). Numerical values in 2, 4,
or 8-byte items are encoded in little-endian.

  - Control:
      - *hash\_type* (1).
  - Transaction data:
      - *nVersion* (4): the *nVersion* of the transaction.
      - *nLockTime* (4): the *nLockTime* of the transaction.
      - If *hash\_type & 0xc0* is zero:
          - *sha\_prevouts* (32): the SHA256 of the serialization of all
            input outpoints.
          - *sha\_amounts* (32): the SHA256 of the serialization of all
            spent output amounts.
          - *sha\_scriptpubkeys* (32): the SHA256 of the serialization
            of all spent output *scriptPubKey*s.
          - *sha\_sequences* (32): the SHA256 of the serialization of
            all input *nSequence*.
      - If *hash\_type & 3* does not equal `SIGHASH_NONE` or
        `SIGHASH_SINGLE`:
          - *sha\_outputs* (32): the SHA256 of the serialization of all
            outputs in `CTxOut` format.
  - Data about this input:
      - *spend\_type* (1): equal to *(ext\_flag \* 2) + annex\_present*,
        where *annex\_present* is 0 if no annex is present, or 1
        otherwise (the original witness stack has two or more witness
        elements, and the first byte of the last element is *0x50*)
      - If *hash\_type & 0xc0* is non-zero:
          - If *hash\_type & 0xc0* is `SIGHASH_ANYONECANPAY`:
              - *outpoint* (36): the `COutPoint` of this input (32-byte
                hash + 4-byte little-endian).
          - If *hash\_type & 0xc0* is `SIGHASH_ANYONECANPAY` or
            `SIGHASH_ANYPREVOUT`:
              - *amount* (8): value of the previous output spent by this
                input.
              - *scriptPubKey* (35): *scriptPubKey* of the previous
                output spent by this input, serialized as script inside
                `CTxOut`. Its size is always 35 bytes.
          - *nSequence* (4): *nSequence* of this input.
      - If *hash\_type & 0xc0* is zero:
          - *input\_index* (4): index of this input in the transaction
            input vector. Index of the first input is 0.
      - If an annex is present (the lowest bit of *spend\_type* is set):
          - *sha\_annex* (32): the SHA256 of *(compact\_size(size of
            annex) || annex)*, where *annex* includes the mandatory
            *0x50* prefix.
  - Data about this output:
      - If *hash\_type & 3* equals `SIGHASH_SINGLE`:
          - *sha\_single\_output* (32): the SHA256 of the corresponding
            output in `CTxOut` format.

Similarly, *SigExt118(hash\_type,key\_version)* evaluates to the
concatenation of:

  - Extension:
      - If *hash\_type & 0xc0* is not `SIGHASH_ANYPREVOUTANYSCRIPT`:
          - *tapleaf\_hash* (32): the tapleaf hash as defined in [BIP
            341](bip-0341.mediawiki "wikilink")
      - *key\_version* (1).
      - *codesep\_pos* (4): the opcode position of the last executed
        `OP_CODESEPARATOR` before the currently executed signature
        opcode, with the value in little endian (or *0xffffffff* if none
        executed). The first opcode in a script has a position of 0. A
        multi-byte push opcode is counted as one opcode, regardless of
        the size of data being pushed.

Note that if *hash\_type & 0x40* is zero,
*SigMsg118(hash\_type,ext\_flag) == SigMsg(hash\_type,ext\_flag)*, and
*SigExt118(hash\_type,0x00) == ext* (where *ext* is the message
extension as defined in [BIP 342](bip-0342.mediawiki "wikilink")).

To verify a signature *sig* for a BIP 118 public key *p*:

  - If the *sig* is 64 bytes long, return *Verify(p,
    hash<sub>TapSigHash</sub>(0x00 || SigMsg118(0x00, 1) ||
    SigExt118(0x00, 0x02), sig)*, where *Verify* is defined in [BIP
    340](bip-0340.mediawiki "wikilink").
  - If the *sig* is 65 bytes long, return *sig\[64\] ≠ 0x00 and
    Verify(p, hash<sub>TapSighash</sub>(0x00 || SigMsg118(sig\[64\], 1)
    || SigExt118(sig\[64\], 0x02), sig\[0:64\])*.
  - Otherwise, fail.

The key differences from [BIP 342](bip-0342.mediawiki "wikilink")
signature verification are:

  - In all cases, `key_version` is set to the constant value `0x01`
    instead of `0x00`.\[6\]
  - If `SIGHASH_ANYPREVOUT` is set, the digest is calculated as if
    `SIGHASH_ANYONECANPAY` was set, except `outpoint` is not included in
    the digest.
  - If `SIGHASH_ANYPREVOUTANYSCRIPT` is set, the digest is calculated as
    if `SIGHASH_ANYONECANPAY` was set, except `outpoint`, `scriptPubKey`
    and `tapleaf_hash` are not included in the digest.

## Security

#### Signature replay

By design, `SIGHASH_ANYPREVOUT` and `SIGHASH_ANYPREVOUTANYSCRIPT`
introduce additional potential for signature replay (that is they allow
the same signature to be reused on a different transaction) when
compared to `SIGHASH_ALL` and `SIGHASH_ANYONECANPAY` signatures.

Both `SIGHASH_ALL` and `SIGHASH_ANYONECANPAY` signatures prevent
signature replay by committing to one or more inputs, so replay of the
signature is only possible if the same input can be spent multiple
times, which is not possible on the Bitcoin blockchain (due to
enforcement of [BIP 30](bip-0030.mediawiki "wikilink")). With
`SIGHASH_ANYPREVOUT` signature replay is possible for different UTXOs
with the same `scriptPubKey` and the same value, while with
`SIGHASH_ANYPREVOUTANYSCRIPT` signature replay is possible for any UTXOs
that reuse the same BIP 118 public key in one of their potential
scripts.

As a consequence, implementors MUST ensure that BIP 118 public keys are
only reused when signature replay cannot cause loss of funds (eg due to
other features of the protocol or other constraints on the transaction),
or when such a loss of funds is acceptable.

#### Malleability

Use of `SIGHASH_ANYPREVOUT` or `SIGHASH_ANYPREVOUTANYSCRIPT` may
introduce additional malleability vectors.

In particular, a transaction authenticated using only ANYPREVOUT
signatures is malleable to anyone able to provide an alternate input
satisfied by the signature -- an input changed in this way would produce
a new, valid transaction paying the same recipient, but with a different
txid. Depending on the changes to the inputs, this might conflict with
the original transaction (if some inputs remain shared) or might result
in a double-payment to the recipient (if they do not).

Further, for a chain of transactions using the same `scriptPubKey` and
value, and only authenticated via ANYPREVOUT signatures (as envisioned
in eltoo for failure cases), it may be possible for any third party to
malleate the transactions (and their txids) without having access to any
of the private keys, particularly by omitting intermediate transactions.

This form of malleation can be dealt with by the child transactions also
using ANYPREVOUT signatures -- when a parent transaction is malleated,
its children can be adjusted to reference the new txid as the input and
the ANYPREVOUT signatures remain valid.

However child transactions that are authorised by a `SIGHASH_ALL` or
`SIGHASH_ANYONECANPAY` signature will need new signatures if their
inputs are malleated in this way. This risk may be mitigated somewhat by
using [BIP 68](bip-0068.mediawiki "wikilink")/[BIP
112](bip-0112.mediawiki "wikilink") relative time locks before spending
a UTXO that had been authorised via an ANYPREVOUT signature with
`SIGHASH_ALL` or `SIGHASH_ANYONECANPAY`: a relative timelock can ensure
that the inputs have enough confirmations that they can only be replaced
in the event of a large block reorg. Note that this approach has
drawbacks: relative timelocks prevent fee-bumping via
child-pays-for-parent, and have the obvious drawback of making the funds
temporarily unusable until the timelock expires.

#### Privacy considerations

It is expected that ANYPREVOUT signatures will only be rarely used in
practice. Protocol and wallet designers should aim to have their
transactions use Taproot key path spends whenever possible, both for
efficiency reasons due to the lower transaction weight, but also for
privacy reasons to avoid third parties being able to distinguish their
transactions from those of other protocols.

Transactions that do use ANYPREVOUT signatures will therefore reveal
information about the transaction, potentially including that
cooperation was impossible, or what protocol or software was used (due
to the details of the script).

In order to maximise privacy, it is therefore recommended that protocol
designers only use BIP 118 public keys in scripts that will be spent
using at least one ANYPREVOUT signature, and either use key path spends
or alternate scripts in the taproot merkle tree for any spends that can
be authorised without ANYPREVOUT signatures. Following this
recommendation may require additional script branches, which may mean
disregarding this recommendation may result in a better tradeoff between
cost and privacy in some circumstances.

## Rationale

<references />

## Deployment

TODO

This may be deployed as a soft-fork either concurrent with, or
subsequent to the deployment of [BIP
340](bip-0340.mediawiki "wikilink"), [BIP
341](bip-0341.mediawiki "wikilink") and [BIP
342](bip-0342.mediawiki "wikilink").

## Backwards compatibility

As a soft fork, older software will continue to operate without
modification. Nodes that have not upgraded to support [BIP
341](bip-0341.mediawiki "wikilink") will see all taproot witness
programs as anyone-can-spend scripts, and nodes that have upgraded to
support [BIP 341](bip-0341.mediawiki "wikilink") and [BIP
342](bip-0342.mediawiki "wikilink") but not BIP 118 will simply treat
any non-empty signature against a BIP 118 public key as valid. As such,
nodes are strongly encourage to upgrade in order to fully validate
signatures for the new public key type.

Non-upgraded wallets can receive and send bitcoin from non-upgraded and
upgraded wallets using SegWit version 0 programs, traditional
pay-to-pubkey-hash, etc. Depending on the implementation, non-upgraded
wallets may be able to send to SegWit version 1 programs if they support
sending to [BIP350](bip-0350.mediawiki "wikilink") Bech32m addresses and
do not prevent the transaction from being broadcast due to considering
the outputs non-standard.

## Revisions

Apart from being based on Taproot rather than SegWit v0, the main
differences to prior revisions of this BIP are:

  - The sighash flag has been renamed from "NOINPUT" to "ANYPREVOUT" to
    reflect that while any prevout may potentially be used with the
    signature, some aspects of the input are still committed to, namely
    the input nSequence value, and (optionally) the spending conditions
    and amount.
  - Previously NOINPUT would have worked for direct public key spends
    (assuming deployment was fleshed out in a way similar to BIP 141
    P2WPKH and P2WSH), however this proposal only applies to signatures
    via tapscript, and not direct key path spends. This means that
    addresses must opt-in to the ability to be spent by a
    `SIGHASH_ANYPREVOUT` or `SIGHASH_ANYPREVOUTANYSCRIPT` signature by
    including an appropriate tapscript path when the address is created.
  - NOINPUT signatures do not commit to the output's spending conditions
    either via `scriptPubKey` or the redeem/witness script. This
    behaviour is preserved when `SIGHASH_ANYPREVOUTANYSCRIPT` is used,
    but when `SIGHASH_ANYPREVOUT` is used, the signature now commits to
    `scriptPubKey` and the tapscript.
  - NOINPUT signatures did commit to the input's amount. This behaviour
    is preserved when `SIGHASH_ANYPREVOUT` is used, but not when
    `SIGHASH_ANYPREVOUTANYSCRIPT` is used.
  - `OP_CODESEPARATOR` in script will affect both `SIGHASH_ANYPREVOUT`
    and `SIGHASH_ANYPREVOUTANYSCRIPT` signatures, whereas it would not
    have in the previous draft.

## Acknowledgements

The `SIGHASH_NOINPUT` flag was first proposed by Joseph Poon in
[February 2016](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2016-February/012460.html),
after being mentioned in the original [Lightning
paper](http://lightning.network/lightning-network-paper.pdf) by Joseph
Poon and Thaddeus Dryja. This document is the result of discussions with
many people and had direct input from Greg Maxwell, Jonas Nick, Pieter
Wuille and others.

1.  **Why a new public key type?** New public key types for tapscript
    can be introduced in a soft fork by specifying new rules for
    *unknown public key types* as specified in [BIP
    342](bip-0342.mediawiki "wikilink"), as this only requires adding
    restrictions to the pre-existing signature opcodes. Possible
    alternative approaches would be to define new script opcodes, to use
    a different taproot leaf version, or to use a different set of
    SegWit outputs than those specified by [BIP
    341](bip-0341.mediawiki "wikilink"); however all of these approaches
    are more complicated, and are better reserved for other upgrades
    where the additional flexibility is actually needed. In this case,
    we specify a new transaction digest, but retain the same elliptic
    curve and signature algorithm (ie, secp256k1 and [BIP
    340](bip-0340.mediawiki "wikilink")).
2.  **Why (and why not) commit to the witness script?** The
    [eltoo](https://blockstream.com/eltoo.pdf) paper provides an example
    of why committing to the witness script is not always appropriate.
    It uses script and the transaction `nLockTime` to make signatures
    asymmetric, so that a transaction with an earlier signature can be
    spent by a transaction with a later signature, but a transaction
    with a later signature cannot be spent by a transaction with an
    earlier signature. As a result, a single signature for a third, even
    later transaction must be able to spend both the prior transactions,
    even though they have a different tapscript. On the other hand,
    these cases also provide a good reason to have the option to commit
    to the script: because each transaction has a new script, committing
    to the script allows you to produce a signature that applies to
    precisely one of these transactions. In the eltoo case, this allows
    you to have a signature for an update transaction that can be
    applied to any prior update, and a signature for a settlement
    transaction that applies only to the corresponding update
    transaction, while using the same key for both, which in turn allows
    for a more compact script.
3.  **Why (and why not) commit to the input value?** Committing to the
    input value may provide additional safety that a signature can't be
    maliciously reused to claim funds that the signer does not intend to
    spend, so by default it seems sensible to commit to it. However,
    doing so prevents being able to use a single signature to
    consolidate a group of UTXOs with the same spending condition into a
    single UTXO which may be useful for some protocols, such as the
    proposal for [layered commitments with
    eltoo](https://lists.linuxfoundation.org/pipermail/lightning-dev/2020-January/002448.html).
4.  **What about key path spends?** This proposal only supports
    ANYPREVOUT signatures via script path spends, and does not support
    ANYPREVOUT signatures for key path spends. This is for two reasons:
    first, not supporting key path spends allows this proposal to be
    independent of the core changes included in [BIP
    341](bip-0341.mediawiki "wikilink") and [BIP
    342](bip-0342.mediawiki "wikilink"); second, it allows addresses to
    opt-in or opt-out of ANYPREVOUT support while remaining
    indistinguishable prior to being spent.
5.  **Use of 0x01 public key type** Because `OP_0` leaves an empty
    vector on the stack it would not satisfy [BIP
    342](bip-0342.mediawiki "wikilink")'s rules for unknown public key
    types. As such, it is convenient to use one of `OP_1..OP_16` or
    `OP_1NEGATE` as a way to reference the taproot internal key. To keep
    things as simple as possible, we use the first of these, and add the
    same byte as a prefix to allow ANYPREVOUT signatures for explicitly
    specified keys.
6.  **Why change key\_version?** Changing `key_version` ensures that if
    the same private key is used to generate both a [BIP
    342](bip-0342.mediawiki "wikilink") key and a BIP 118 public key,
    that a signature for the [BIP 342](bip-0342.mediawiki "wikilink")
    key is not also valid for the BIP 118 public key (and vice-versa).
