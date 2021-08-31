+++
title = "Multisig Output Script Descriptors"
date = 2021-06-27
weight = 383
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 383
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0383.mediawiki"
+++

``` 
  BIP: 383
  Layer: Applications
  Title: Multisig Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0383
  Status: Draft
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
```

## Abstract

This document specifies `multi()`, and `sortedmulti()` output script
descriptors. Both functions take a threshold and one or more public keys
and produce a multisig output script. `multi()` specifies the public
keys in the output script in the order given in the descriptor while
`sortedmulti()` sorts the public keys lexicographically when the output
script is produced.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

The most common complex script used in Bitcoin is a threshold multisig.
These expressions allow specifying multisig scripts as a descriptor.

## Specification

Two new script expressions are defined: `multi()`, and `sortedmulti()`.
Both expressions produce the scripts of the same template and take the
same arguments. They are written as `multi(k,KEY_1,KEY_2,...,KEY_n)`.
`k` is the threshold - the number of keys that must sign the input for
the script to be valid. `KEY_1,KEY_2,...,KEY_n` are the key expressions
for the multisig. `k` must be less than or equal to `n`.

`multi()` and `sortedmulti()` expressions can be used as a top level
expression, or inside of either a `sh()` or `wsh()` descriptor.
Depending on the higher level descriptors, there may be restrictions on
the type of public keys that can be included.

Depending on the higher level descriptors, there are also restrictions
on the number of keys that can be present, i.e. the maximum value of
`n`. When used at the top level, there can only be at most 3 keys. When
used inside of a `sh()` expression, there can only be most 15 compressed
public keys (this is limited by the P2SH script limit). Otherwise the
maximum number of keys is 20.

The output script produced also depends on the value of `k`. If `k` is
less than or equal to 16:

    OP_k KEY_1 KEY_2 ... KEY_n OP_CHECKMULTISIG

if `k` is greater than 16:

    k KEY_1 KEY_2 ... KEY_n OP_CHECKMULTISIG

### `sortedmulti()`

The only change for `sortedmulti()` is that the keys are sorted
lexicographically prior to the creation of the output script. This
sorting is on the keys that are to be put into the output script, i.e.
after all extended keys are derived.

### Multiple Extended Keys</tt>

When one or more the key expressions in a `multi()` or `sortedmulti()`
expression are extended keys, the derived keys use the same child index.
This changes the keys in lockstep and allows for output scripts to be
indexed in the same way that the derived keys are indexed.

## Test Vectors

TBD

## Backwards Compatibility

`multi()`, and `sortedmulti()` descriptors use the format and general
operation specified in [380](bip-0380.mediawiki "wikilink"). As these
are a wholly new descriptors, they are not compatible with any
implementation. However the scripts produced are standard scripts so
existing software are likely to be familiar with them.

## Reference Implemntation

`multi()`, and `multi()` descriptors have been implemented in Bitcoin
Core since version 0.17.
