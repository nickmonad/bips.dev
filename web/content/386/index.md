+++
title = "tr() Output Script Descriptors"
date = 2021-06-27
weight = 386
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 386
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0386.mediawiki"
+++

      BIP: 386
      Layer: Applications
      Title: tr() Output Script Descriptors
      Author: Pieter Wuille <pieter@wuille.net>
              Andrew Chow <andrew@achow101.com>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0386
      Status: Draft
      Type: Informational
      Created: 2021-06-27
      License: BSD-2-Clause

## Abstract

This document specifies `tr()` output script descriptors. `tr()`
descriptors take a key and optionally a tree of scripts and produces a
P2TR output script.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

Taproot added one additional standard output script format: P2TR. These
expressions allow specifying those formats as a descriptor.

## Specification

A new script expression is defined: `tr()`. A new expression is defined:
Tree Expressions

### Tree Expression

A Tree Expression (denoted `TREE`) is an expression which represents a
tree of scripts. The way the tree is represented in an output script is
dependent on the higher level expressions.

A Tree Expression is:

- Any Script Expression that is allowed at the level this Tree
  Expression is in.
- A pair of Tree Expressions consisting of:
  - An open brace `{`
  - A Tree Expression
  - A comma `,`
  - A Tree Expression
  - A closing brace `}`

### `tr()`

The `tr(KEY)` or `tr(KEY, TREE)` expression can only be used as a top
level expression. All key expressions under any `tr()` expression must
create x-only public keys.

`tr(KEY)` takes a single key expression as an argument and produces a
P2TR output script which does not have a script path. Each key produced
by the key expression is used as the internal key of a P2TR output as
specified by [BIP 341](/341).
Specifically, "If the spending conditions do not require a script path,
the output key should commit to an unspendable script path instead of
having no script path. This can be achieved by computing the output key
point as *Q = P + int(hash<sub>TapTweak</sub>(bytes(P)))G*."

    internal_key:       lift_x(KEY)
    32_byte_output_key: internal_key + int(HashTapTweak(bytes(internal_key)))G
    scriptPubKey:       OP_1 <32_byte_output_key>

`tr(KEY, TREE)` takes a key expression as the first argument, and a tree
expression as the second argument and produces a P2TR output script
which has a script path. The keys produced by the first key expression
are used as the internal key as specified by [BIP
341](/341).
The Tree expression becomes the Taproot script tree as described in BIP
341. A merkle root is computed from this tree and combined with the
internal key to create the Taproot output key.

    internal_key:       lift_x(KEY)
    merkle_root:        HashTapBranch(TREE)
    32_byte_output_key: internal_key + int(HashTapTweak(bytes(internal_key) || merkle_root))G
    scriptPubKey:       OP_1 <32_byte_output_key>

### Modified Key Expression

Key Expressions within a `tr()` expression must only create x-only
public keys. Uncompressed public keys are not allowed, but compressed
public keys would be implicitly converted to x-only public keys. The
keys derived from extended keys must be serialized as x-only public
keys. An additional key expression is defined only for use within a
`tr()` descriptor:

- A 64 hex character string representing an x-only public key

## Test Vectors

TBD

## Backwards Compatibility

`tr()` descriptors use the format and general operation specified in
[380](/380). As these are a set of wholly new
descriptors, they are not compatible with any implementation. However
the scripts produced are standard scripts so existing software are
likely to be familiar with them.

Tree Expressions are largely incompatible with existing script
expressions due to the restrictions in those expressions. As of
2021-06-27, the only allowed script expression that can be used in a
tree expression is `pk()`. However there will be future BIPs that
specify script expressions that can be used in tree expressions.

## Reference Implementation

`tr()` descriptors have been implemented in Bitcoin Core since version
22.0.
