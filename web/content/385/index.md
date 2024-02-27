+++
title = "raw() and addr() Output Script Descriptors"
date = 2021-06-27
weight = 385
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Ava Chow"]
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
          Ava Chow <me@achow101.com>
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

Valid descriptors followed by the scripts they produce.

  - `raw(deadbeef)`
      - `deadbeef`
  - `raw(512103a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd4104a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea23552ae)`
      - `512103a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd4104a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea23552ae`
  - `raw(a9149a4d9901d6af519b2a23d4a2f51650fcba87ce7b87)`
      - `a9149a4d9901d6af519b2a23d4a2f51650fcba87ce7b87`
  - `addr(3PUNyaW7M55oKWJ3kDukwk9bsKvryra15j)`
      - `a914eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee87`

Invalid descriptors

  - Non-hex script: `raw(asdf)`
  - Invalid address: `addr(asdf)`
  - `raw` nested in `sh`: `sh(raw(deadbeef))`
  - `raw` nested in `wsh`: `wsh(raw(deadbeef))`
  - `addr` nested in `sh`:
    `sh(addr(3PUNyaW7M55oKWJ3kDukwk9bsKvryra15j))`
  - `addr` nested in `wsh`:
    `wsh(addr(3PUNyaW7M55oKWJ3kDukwk9bsKvryra15j))`

## Backwards Compatibility

`raw()` and `addr()` descriptors use the format and general operation
specified in [380](/380). As this is a wholly
new descriptor, it is not compatible with any implementation. The reuse
of existing Bitcoin addresses allows for this to be more easily
implemented.

## Reference Implementation

`raw()` and `addr()` descriptors have been implemented in Bitcoin Core
since version 0.17.
