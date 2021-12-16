+++
title = "combo() Output Script Descriptors"
date = 2021-06-27
weight = 384
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 384
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0384.mediawiki"
+++

``` 
  BIP: 384
  Layer: Applications
  Title: combo() Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0384
  Status: Draft
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
```

## Abstract

This document specifies `combo()` output script descriptors. These take
a key and produce P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH output scripts if
applicable to the key.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

In order to make the transition from traditional key based wallets to
descriptor based wallets easier, it is useful to be able to take a key
and produce the scripts which have traditionally been produced by wallet
software.

## Specification

A new top level script expression is defined: `combo(KEY)`. This
expression can only be used as a top level expression. It takes a single
key expression as an argument and produces either 2 or 4 output scripts,
depending on the key. A `combo()` expression always produces a P2PK and
P2PKH script, the same as putting the key in both a `pk()` and a `pkh()`
expression. If the key is/has a compressed public key, then P2WPKH and
P2SH-P2WPKH scripts are also produced, the same as putting the key in
both a `wpkh()` and `sh(wpkh())` expression.

## Test Vectors

TBD

## Backwards Compatibility

`combo()` descriptors use the format and general operation specified in
[380](bip-0380.mediawiki "wikilink"). As this is a wholly new
descriptor, it is not compatible with any implementation. However the
scripts produced are standard scripts so existing software are likely to
be familiar with them.

## Reference Implementation

`combo()` descriptors have been implemented in Bitcoin Core since
version 0.17.
