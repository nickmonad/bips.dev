+++
title = "Dealing with signature encoding malleability"
date = 2016-08-16
weight = 146
in_search_index = true

[extra]
bip = 146
status = "Withdrawn"
+++

      BIP: 146
      Layer: Consensus (soft fork)
      Title: Dealing with signature encoding malleability
      Author: Johnson Lau <jl2012@xbt.hk>
              Pieter Wuille <pieter.wuille@gmail.com>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0146
      Status: Withdrawn
      Type: Standards Track
      Created: 2016-08-16
      License: PD

## Abstract

This document specifies proposed changes to the Bitcoin transaction
validity rules to fix signature malleability related to ECDSA signature
encoding.

## Motivation

Signature malleability refers to the ability of any relay node on the
network to transform the signature in transactions, with no access to
the relevant private keys required. For non-segregated witness
transactions, signature malleability will change the `txid` and
invalidate any unconfirmed child transactions. Although the `txid` of
segregated witness
([BIP141](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki))
transactions is not third party malleable, this malleability vector will
change the `wtxid` and may reduce the efficiency of compact block relay
([BIP152](https://github.com/bitcoin/bips/blob/master/bip-0152.mediawiki)).

Since the enforcement of Strict DER signatures
([BIP66](https://github.com/bitcoin/bips/blob/master/bip-0066.mediawiki)),
there are 2 remaining known sources of malleability in ECDSA signatures:

1.  **Inherent ECDSA signature malleability**: ECDSA signatures are
    inherently malleable as taking the negative of the number S inside
    (modulo the curve order) does not invalidate it.

<!-- -->

1.  **Malleability of failing signature**: If a signature failed to
    validate in `OP_CHECKSIG` or `OP_CHECKMULTISIG`, a `FALSE` would be
    returned to the stack and the script evaluation would continue. The
    failing signature may take any value, as long as it follows all the
    rules described in BIP66.

This document specifies new rules to fix the aforesaid signature
malleability.

## Specification

To fix signature encoding malleability, the following new rules are
applied to pre-segregated witness and segregated witness scripts:

### LOW\_S

We require that the S value inside ECDSA signatures is at most the curve
order divided by 2 (essentially restricting this value to its lower half
range). Every signature passed to `OP_CHECKSIG`[1], `OP_CHECKSIGVERIFY`,
`OP_CHECKMULTISIG`, or `OP_CHECKMULTISIGVERIFY`, to which ECDSA
verification is applied, MUST use a S value between `0x1` and
`0x7FFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF 5D576E73 57A4501D DFE92F46 681B20A0`
(inclusive) with strict DER encoding (see
[BIP66](https://github.com/bitcoin/bips/blob/master/bip-0066.mediawiki)).

If a signature passing to ECDSA verification does not pass the Low S
value check and is not an empty byte array, the entire script evaluates
to false immediately.

A high S value in signature could be trivially replaced by
`S' = 0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141 - S`.

### NULLFAIL

If an `OP_CHECKSIG` is trying to return a `FALSE` value to the stack, we
require that the relevant signature must be an empty byte array.

If an `OP_CHECKMULTISIG` is trying to return a `FALSE` value to the
stack, we require that all signatures passing to this `OP_CHECKMULTISIG`
must be empty byte arrays, even the processing of some signatures might
have been skipped due to early termination of the signature
verification.

Otherwise, the entire script evaluates to false immediately.

## Examples

The following examples are the combined results of the LOW\_S and
NULLFAIL rules.[2]

Notation:

` CO       : curve order = 0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141`  
` HCO      : half curve order = CO / 2 = 0x7FFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF 5D576E73 57A4501D DFE92F46 681B20A0`  
` P1, P2   : valid, serialized, public keys`  
` S1L, S2L : valid low S value signatures using respective keys P1 and P2 (1 ≤ S ≤ HCO)`  
` S1H, S2H : signatures with high S value (otherwise valid) using respective keys P1 and P2 (HCO < S < CO)`  
` F        : any BIP66-compliant non-empty byte array but not a valid signature`

These scripts will return a `TRUE` to the stack as before:

` S1L P1 CHECKSIG`  
` 0 S1L S2L 2 P1 P2 2 CHECKMULTISIG`

These scripts will return a `FALSE` to the stack as before:

` 0 P1 CHECKSIG`  
` 0 0 0 2 P1 P2 2 CHECKMULTISIG`

These previously `TRUE` scripts will fail immediately under the new
rules:

` S1H P1 CHECKSIG`  
` 0 S1H S2L 2 P1 P2 2 CHECKMULTISIG`  
` 0 S1L S2H 2 P1 P2 2 CHECKMULTISIG`  
` 0 S1H S2H 2 P1 P2 2 CHECKMULTISIG`

These previously `FALSE` scripts will fail immediately under the new
rules:

` F P1 CHECKSIG`  
` 0 S2L S1L 2 P1 P2 2 CHECKMULTISIG`  
` 0 S1L F   2 P1 P2 2 CHECKMULTISIG`  
` 0 F   S2L 2 P1 P2 2 CHECKMULTISIG`  
` 0 S1L 0   2 P1 P2 2 CHECKMULTISIG`  
` 0 0   S2L 2 P1 P2 2 CHECKMULTISIG`  
` 0 F   0   2 P1 P2 2 CHECKMULTISIG`  
` 0 0   F   2 P1 P2 2 CHECKMULTISIG`

## Deployment

This BIP will be deployed by "version bits"
[BIP9](https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki).
Details TBD.

For Bitcoin mainnet, the BIP9 starttime will be midnight TBD UTC (Epoch
timestamp TBD) and BIP9 timeout will be midnight TBD UTC (Epoch
timestamp TBD).

For Bitcoin testnet, the BIP9 starttime will be midnight TBD UTC (Epoch
timestamp TBD) and BIP9 timeout will be midnight TBD UTC (Epoch
timestamp TBD).

## Compatibility

The reference client has produced LOW\_S compatible signatures since
v0.9.0, and the LOW\_S rule has been enforced as relay policy by the
reference client since v0.11.1. As of August 2016, very few transactions
violating the requirement are being added to the chain. For all
scriptPubKey types in actual use, non-compliant signatures can trivially
be converted into compliant ones, so there is no loss of functionality
by these requirements.

Scripts with failing `OP_CHECKSIG` or `OP_CHECKMULTISIG` rarely happen
on the chain. The NULLFAIL rule has been enforced as relay policy by the
reference client since v0.13.1.

Users MUST pay extra attention to these new rules when designing exotic
scripts.

## Implementation

Implementations for the reference client is available at:

<https://github.com/bitcoin/bitcoin/blob/35fe0393f216aa6020fc929272118eade5628636/src/script/interpreter.cpp#L185>

and

<https://github.com/bitcoin/bitcoin/pull/8634>

## Footnotes

<references />

## Acknowledgements

This document is extracted from the previous
[BIP62](https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki)
proposal which had input from various people.

## Copyright

This document is placed in the public domain.

[1] Including pay-to-witness-public-key-hash (P2WPKH) described in
BIP141

[2] Please note that due to implementation details in reference client
v0.13.1, some signatures with S value higher than the half curve order
might pass the LOW\_S test. However, such signatures are certainly
invalid, and will fail later due to NULLFAIL test.
