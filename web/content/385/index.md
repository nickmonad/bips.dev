+++
title = "raw() and addr() Output Script Descriptors"
date = 2021-06-27
weight = 385
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Andrew Chow"]
status = ["Draft"]

[extra]
bip = 385
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0385.mediawiki"
+++

``` 
  BIP: 385
  Layer: Applications
  Title: raw() and addr() Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0385
  Status: Draft
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
```

## Abstract

This document specifies `raw()` and `addr()` output script descriptors.
`raw()` encapsulates a raw script as a descriptor. `addr()` encapsulates
an address as a descriptor.

## Copyright

This BIP is licensed under the BSD 2-clause license.

## Motivation

In order to make descriptors maximally compatible with scripts in use
today, it is useful to be able to wrap any arbitrary output script or an
address into a descriptor.

## Specification

Two new script expressions are defined: `raw()` and `addr()`.

### `raw()`

The `raw(HEX)` expression can only be used as a top level descriptor. As
the argument, it takes a hex string representing a Bitcoin script. The
output script produced by this descriptor is the script represented by
`HEX`.

### `addr()`

The `addr(ADDR)` expression can only be used as a top level descriptor.
It takes an address as its single argument. The output script produced
by this descriptor is the output script produced by the address `ADDR`.

## Test Vectors

TBD

## Backwards Compatibility

`raw()` and `addr()` descriptors use the format and general operation
specified in [380](bip-0380.mediawiki "wikilink"). As this is a wholly
new descriptor, it is not compatible with any implementation. The reuse
of existing Bitcoin addresses allows for this to be more easily
implemented.

## Reference Implementation

`raw()` and `addr()` descriptors have been implemented in Bitcoin Core
since version 0.17.
