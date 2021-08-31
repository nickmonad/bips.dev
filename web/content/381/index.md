+++
title = "Non-Segwit Output Script Descriptors"
date = 2021-06-27
weight = 381
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 381
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0381.mediawiki"
+++

``` 
  BIP: 381
  Layer: Applications
  Title: Non-Segwit Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0381
  Status: Draft
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
```

## Abstract

This document specifies `pk()`, `pkh()`, and `sh()` output script
descriptors. `pk()` descriptors take a key and produces a P2PK output
script. `pkh()` descriptors take a key and produces a P2PKH output
script. `sh()` descriptors take a script and produces a P2SH output
script.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

Prior to the activation of Segregated Witness, there were 3 main
standard output script formats: P2PK, P2PKH, and P2SH. These expressions
allow specifying those formats as a descriptor.

## Specification

Three new script expressions are defined: `pk()`, `pkh()`, and `sh()`.

### `pk()`

The `pk(KEY)` expression can be used in any context or level of a
descriptor. It takes a single key expression as an argument and produces
a P2PK output script. Depending on the higher level descriptors, there
may be restrictions on the type of public keys that can be included.
Such restrictions will be specified by those descriptors.

The output script produced is:

    <KEY> OP_CHECKSIG

### `pkh()`

The `pkh(KEY)` expression can be used as a top level expression, or
inside of either a `sh()` or `wsh()` descriptor. It takes a single key
expression as an argument and produces a P2PKH output script. Depending
on the higher level descriptors, there may be restrictions on the type
of public keys that can be included. Such restrictions will be specified
by those descriptors.

The output script produced is:

    OP_DUP OP_HASH160 <KEY_hash160> OP_EQUALVERIFY OP_CHECKSIG

### `sh()`

The `sh(SCRIPT)` expression can only be used as a top level expression.
It takes a single script expression as an argument and produces a P2SH
output script. `sh()` expressions also create a redeemScript which is
required in order to spend outputs which use its output script. This
redeemScript is the output script produced by the `SCRIPT` argument to
`sh()`.

The output script produced is:

    OP_HASH160 <SCRIPT_hash160> OP_EQUAL

## Test Vectors

TBD

## Backwards Compatibility

`pk()`, `pkh()`, and `sh()` descriptors use the format and general
operation specified in [380](bip-0380.mediawiki "wikilink"). As these
are a wholly new descriptors, they are not compatible with any
implementation. However the scripts produced are standard scripts so
existing software are likely to be familiar with them.

## Reference Implemntation

`pk()`, `pkh()`, and `sh()` descriptors have been implemented in Bitcoin
Core since version 0.17.
