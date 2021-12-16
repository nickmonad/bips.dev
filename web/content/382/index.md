+++
title = "Segwit Output Script Descriptors"
date = 2021-06-27
weight = 382
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 382
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0382.mediawiki"
+++

``` 
  BIP: 382
  Layer: Applications
  Title: Segwit Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0382
  Status: Draft
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
```

## Abstract

This document specifies `wpkh()`, and `wsh()` output script descriptors.
`wpkh()` descriptors take a key and produces a P2WPKH output script.
`wsh()` descriptors take a script and produces a P2WSH output script.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

Segregated Witness added 2 additional standard output script formats:
P2WPKH and P2WSH. These expressions allow specifying those formats as a
descriptor.

## Specification

Two new script expressions are defined: `wpkh()`, and `wsh()`.

### `wpkh()`

The `wpkh(KEY)` expression can be used as a top level expression, or
inside of a `sh()` descriptor. It takes a single key expression as an
argument and produces a P2WPKH output script. Only keys which are/has
compressed public keys can be contained in a `wpkh()` expression.

The output script produced is:

    OP_0 <KEY_hash160>

### `wsh()`

The `wsh(SCRIPT)` expression can be used as a top level expression, or
inside of a `sh()` descriptor. It takes a single script expression as an
argument and produces a P2WSH output script. `wsh()` expressions also
create a witnessScript which is required in order to spend outputs which
use its output script. This redeemScript is the output script produced
by the `SCRIPT` argument to `wsh()`. Any key expression found in any
script expression contained by a `wsh()` expression must only produce
compressed public keys.

The output script produced is:

    OP_0 <SCRIPT_sha256>

## Test Vectors

TBD

## Backwards Compatibility

`wpkh()`, and `wsh()` descriptors use the format and general operation
specified in [380](bip-0380.mediawiki "wikilink"). As these are a wholly
new descriptors, they are not compatible with any implementation.
However the scripts produced are standard scripts so existing software
are likely to be familiar with them.

## Reference Implementation

`wpkh()`, and `wsh()` descriptors have been implemented in Bitcoin Core
since version 0.17.
