+++
title = "PSBT Version 2"
date = 2021-01-14
weight = 370
in_search_index = true

[taxonomies]
authors = ["Andrew Chow"]
status = ["Draft"]

[extra]
bip = 370
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0370.mediawiki"
+++

``` 
  BIP: 370
  Layer: Applications
  Title: PSBT Version 2
  Author: Andrew Chow <achow101@gmail.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0370
  Status: Draft
  Type: Standards Track
  Created: 2021-01-14
  License: BSD-2-Clause
```

## Introduction

### Abstract

This document proposes a second version of the Partially Signed Bitcoin
Transaction format described in BIP 174 which allows for inputs and
outputs to be added to the PSBT after creation.

### Copyright

This BIP is licensed under the 2-clause BSD license.

### Motivation

Partially Signed Bitcoin Transaction Version 0 as described in BIP 174
is unable to have new inputs and outputs be added to the transaction.
The fixed global unsigned transaction cannot be changed which prevents
any additional inputs or outputs to be added. PSBT Version 2 is intended
to rectify this problem.

An additional benficial side effect is that all information for a given
input or output will be provided by its <input-map> or <output-map>.
With Version 0, to retrieve all of the information for an input or
output, data would need to be found in two locations: the
<input-map>/<output-map> and the global unsigned transaction. PSBT
Version 2 now moves all related information to one place.

## Specification

PSBT Version 2 (PSBTv2) only specifies new fields and field
inclusion/exclusion requirements.

`PSBT_GLOBAL_UNSIGNED_TX` must be excluded in PSBTv2.
`PSBT_GLOBAL_VERSION` must be included in PSBTv2 and set to version
number 2\[1\].

The new global types for PSBT Version 2 are as follows:

| Name                         | <keytype>                              | <keydata> | <keydata> Description | <valuedata>         | <valuedata> Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Versions Requiring Inclusion | Versions Requiring Exclusion | Versions Allowing Inclusion |
| ---------------------------- | -------------------------------------- | --------- | --------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------- | ---------------------------- | --------------------------- |
| Transaction Version          | `PSBT_GLOBAL_TX_VERSION = 0x02`        | None      | No key data           | `<32-bit uint>`     | The 32-bit little endian signed integer representing the version number of the transaction being created. Note that this is not the same as the PSBT version number specified by the PSBT\_GLOBAL\_VERSION field.                                                                                                                                                                                                                                                                                                                                            | 2                            | 0                            | 2                           |
| Fallback Locktime            | `PSBT_GLOBAL_FALLBACK_LOCKTIME = 0x03` | None      | No key data           | `<32-bit uint>`     | The 32-bit little endian unsigned integer representing the transaction locktime to use if no inputs specify a required locktime.                                                                                                                                                                                                                                                                                                                                                                                                                             |                              | 0                            | 2                           |
| Input Count                  | `PSBT_GLOBAL_INPUT_COUNT = 0x04`       | None      | No key data           | <compact size uint> | Compact size unsigned integer representing the number of inputs in this PSBT.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 2                            | 0                            | 2                           |
| Output Count                 | `PSBT_GLOBAL_OUTPUT_COUNT = 0x05`      | None      | No key data           | <compact size uint> | Compact size unsigned integer representing the number of outputs in this PSBT.                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | 2                            | 0                            | 2                           |
| Transaction Modifiable Flags | `PSBT_GLOBAL_TX_MODIFIABLE = 0x06`     | None      | No key data           | `<8-bit uint>`      | An 8 bit little endian unsigned integer as a bitfield for various transaction modification flags. Bit 0 is the Inputs Modifiable Flag and indicates whether inputs can be modified. Bit 1 is the Outputs Modifiable Flag and indicates whether outputs can be modified. Bit 2 is the Has SIGHASH\_SINGLE flag and indicates whether the transaction has a SIGHASH\_SINGLE signature who's input and output pairing must be preserved. Bit 2 essentially indicates that the Constructor must iterate the inputs to determine whether and how to add an input. |                              | 0                            | 2                           |

The new per-input types for PSBT Version 2 are defined as follows:

| Name                           | <keytype>                                 | <keydata> | <keydata> Description | <valuedata>     | <valuedata> Description                                                                                                                                                             | Versions Requiring Inclusion | Versions Requiring Exclusion | Versions Allowing Inclusion |
| ------------------------------ | ----------------------------------------- | --------- | --------------------- | --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | ---------------------------- | --------------------------- |
| Previous TXID                  | `PSBT_IN_PREVIOUS_TXID = 0x0e`            | None      | No key data           | <txid>          | 32 byte txid of the previous transaction whose output at PSBT\_IN\_OUTPUT\_INDEX is being spent.                                                                                    | 2                            | 0                            | 2                           |
| Spent Output Index             | `PSBT_IN_OUTPUT_INDEX = 0x0f`             | None      | No key data           | `<32-bit uint>` | 32 bit little endian integer representing the index of the output being spent in the transaction with the txid of PSBT\_IN\_PREVIOUS\_TXID.                                         | 2                            | 0                            | 2                           |
| Sequence Number                | `PSBT_IN_SEQUENCE = 0x10`                 | None      | No key data           | `<32-bit uint>` | The 32 bit unsigned little endian integer for the sequence number of this input. If omitted, the sequence number is assumed to be the final sequence number (0xffffffff).           |                              | 0                            | 2                           |
| Required Time-based Locktime   | `PSBT_IN_REQUIRED_TIME_LOCKTIME = 0x11`   | None      | No key data           | `<32-bit uint>` | 32 bit unsigned little endian integer greater than or equal to 500000000 representing the minimum Unix timestamp that this input requires to be set as the transaction's lock time. |                              | 0                            | 2                           |
| Required Height-based Locktime | `PSBT_IN_REQUIRED_HEIGHT_LOCKTIME = 0x12` | None      | No key data           | `<32-bit uiht>` | 32 bit unsigned little endian integer less than 500000000 representing the minimum block height that this input requires to be set as the transaction's lock time.                  |                              | 0                            | 2                           |

The new per-output types for PSBT Version 2 are defined as follows:

<table>
<thead>
<tr class="header">
<th><p>Name</p></th>
<th><p><keytype></p></th>
<th><p><keydata></p></th>
<th><p><keydata> Description</p></th>
<th><p><valuedata></p></th>
<th><p><valuedata> Description</p></th>
<th><p>Versions Requiring Inclusion</p></th>
<th><p>Versions Requiring Exclusion</p></th>
<th><p>Versions Allowing Inclusion</p></th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Output Amount</p></td>
<td><p><code>PSBT_OUT_AMOUNT = 0x03</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><code>&lt;64-bit int&gt;</code></p></td>
<td><p>64 bit signed little endian integer representing the output's amount in satoshis.</p></td>
<td><p>2</p></td>
<td><p>0</p></td>
<td><p>2</p></td>
</tr>
<tr class="even">
<td><p>Output Script</p></td>
<td><p><code>PSBT_OUT_SCRIPT = 0x04</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><tt></p>
<script>
<p></tt></p></td>
<td><p>The script for this output, also known as the scriptPubKey. Must be omitted in PSBTv0. Must be provided in PSBTv2.</p></td>
<td><p>2</p></td>
<td><p>0</p></td>
<td><p>2</p></td>
</tr>
</tbody>
</table>

### Determining Lock Time

The nLockTime field of a transaction is determined by inspecting the
PSBT\_GLOBAL\_FALLBACK\_LOCKTIME and each input's
PSBT\_IN\_REQUIRED\_TIME\_LOCKTIME and
PSBT\_IN\_REQUIRED\_HEIGHT\_LOCKTIME fields. If none of the inputs have
a PSBT\_IN\_REQUIRED\_TIME\_LOCKTIME and
PSBT\_IN\_REQUIRED\_HEIGHT\_LOCKTIME, then
PSBT\_GLOBAL\_FALLBACK\_LOCKTIME must be used. If
PSBT\_GLOBAL\_FALLBACK\_LOCKTIME is not provided, then it is assumed to
be 0.

If one or more inuts have a PSBT\_IN\_REQUIRED\_TIME\_LOCKTIME or
PSBT\_IN\_REQUIRED\_HEIGHT\_LOCKTIME, then the field chosen is the one
which is supported by all of the inputs. This can be determined by
looking at all of the inputs which specify a locktime in either of those
fields, and choosing the field which is present in all of those inputs.
Inputs not specifying a lock time field can take both types of lock
times, as can those that specify both. The lock time chosen is then the
maximum value of the chosen type of lock time.

### Unique Identification

PSBTv2s can be uniquely identified by constructing an unsigned
transaction given the information provided in the PSBT and computing the
transaction ID of that transaction. Since PSBT\_IN\_SEQUENCE can be
changed by Updaters and Combiners, the sequence number in this unsigned
transaction must be set to 0 (not final, nor the sequence in
PSBT\_IN\_SEQUENCE). The lock time in this unsigned transaction must be
computed as described previously.

## Roles

PSBTv2 introduces new roles and modifies some existing roles.

### Creator

In PSBTv2, the Creator initializes the PSBT with 0 inputs and 0 outputs.
The PSBT version number is set to 2. The transaction version number must
be set to at least 2. \[2\] The Creator should also set
PSBT\_GLOBAL\_FALLBACK\_LOCKTIME. If the Creator is not also a
Constructor and will be giving the PSBT to others to add inputs and
outputs, the PSBT\_GLOBAL\_TX\_MODIFIABLE field must be present and and
the Inputs Modifiable and Outputs Modifiable flags set appropriately. If
the Creator is a Constructor and no inputs and outputs will be added by
other entities, PSBT\_GLOBAL\_TX\_MODIFIABLE may be omitted.

### Constructor

This Constructor is only present for PSBTv2. Once a Creator initializes
the PSBT, a constructor will add inputs and outputs. Before any input or
output may be added, the constructor must check the
PSBT\_GLOBAL\_TX\_MODIFIABLE field. Inputs may only be added if the
Inputs Modifiable flag is True. Outputs may only be added if the Outputs
Modifiable flag is True.

When an input or output is added, the corresponding
PSBT\_GLOBAL\_INPUT\_COUNT or PSBT\_GLOBAL\_OUTPUT\_COUNT must be
incremeted to reflect the number of inputs and outputs in the PSBT. When
an input is added, it must have PSBT\_IN\_PREVIOUS\_TXID and
PSBT\_IN\_OUTPUT\_INDEX set. When an output is added, it must have
PSBT\_OUT\_VALUE and PSBT\_OUT\_OUTPUT\_SCRIPT set. If the input has a
required timelock, Constructors must set the requisite timelock field.
If the input has a required time based timelock, then
PSBT\_IN\_REQUIRED\_TIME\_TIMELOCK must be set If the input has a
required height based timelock, then
PSBT\_IN\_REQUIRED\_HEIGHT\_TIMELOCK must be set. If an input has both
types of timelocks, then both may be set. In some cases, an input that
can allow both types, but a particular branch supporting only one type
of timelock will be taken, then the type of timelock that will be used
can be the only one set.

If an input being added specifies a required time lock, then the
Constructor must iterate through all of the existing inputs and ensure
that the time lock types are compatible. Additionally, if during this
iteration, it finds that any inputs have signatures, it must ensure that
the newly added input does not change the transaction's locktime. If the
newly added input has an incompatible time lock, then it must not be
added. If it changes the transaction's locktime when there are existing
signatures, it must not be added.

If the Has SIGHASH\_SINGLE flag is True, then the Constructor must
iterate through the inputs and find the inputs which have signatures
that use SIGHASH\_SINGLE. The same number of inputs and outputs must be
added before those inputs and their corresponding outputs.

A Constructor may choose to declare that no further inputs and outputs
can be added to the transaction by setting the booleans in
PSBT\_GLOBAL\_TX\_MODIFIABLE to False or by removing this field
entirely.

A single entity is likely to be both a Creator and Constructor.

### Updater

For PSBTv2, an Updater can set the sequence number.

### Signer

For PSBTv2s, a signer must update the PSBT\_GLOBAL\_TX\_MODIFIABLE field
after signing inputs so that it accurately reflects the state of the
PSBT. If the Signer added a signature that does not use
SIGHASH\_ANYONECANPAY, the Input Modifiable flag must be set to False.
If the Signer added a signature that does not use SIGHASH\_NONE, the
Outputs Modifiable flag must be set to False. If the Signer added a
signature that uses SIGHASH\_SINGLE, the Has SIGHASH\_SINGLE flag must
be set to True.

### Transaction Extractor

For PSBTv2s, the transaction is constructed using the PSBTv2 fields. The
lock time for this transaction is determined as described in the
Determining Lock Time section. The Extractor should produce a fully
valid, network serialized transaction if all inputs are complete.

## Backwards Compatibility

PSBTv2 shares the same gemeric format as PSBTv0 as defined in BIP 174.
Parsers for PSBTv0 should be able to deserialize PSBTv2 with only
changes to support the new fields.

However PSBTv2 is incompatible with PSBTv0, and vice versa due to the
use of the PSBT\_GLOBAL\_VERSION. This incompatibility is intentional so
that PSBT\_GLOBAL\_UNSIGNED\_TX could be removed in PSBTv2. However it
is possible to convert a PSBTv2 to a PSBTv0 by creating an unsigned
transaction from the PSBTv2 fields.

## Test Vectors

TBD

## Rationale

<references/>

## Reference implementation

The reference implementation of the PSBT format is available at
<https://github.com/achow101/bitcoin/tree/psbt2>.

1.  **What happened to version number 1?** Version number 1 is skipped
    because PSBT Version 0 has been colloquially referred to as version
    1. Originally this BIP was to be version 1, but because it has been
    colloquially referred to as version 2 during its design phrase, it
    was decided to change the version number to 2 so that there would
    not be any confusion
2.  **Why does the transaction version number need to be at least 2?**
    The transaction version number is part of the validation rules for
    some features such as OP\_CHECKSEQUENCEVERIFY. Since it is backwards
    compatible, and there are other ways to disable those features (e.g.
    through sequence numbers), it is easier to require transactions be
    able to support these features than to try to negotiate the
    transaction version number.
