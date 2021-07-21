+++
title = "Taproot Fields for PSBT"
date = 2021-06-21
weight = 371
in_search_index = true

[taxonomies]
authors = ["Andrew Chow"]
status = ["Draft"]

[extra]
bip = 371
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0371.mediawiki"
+++

``` 
  BIP: 371
  Layer: Applications
  Title: Taproot Fields for PSBT
  Author: Andrew Chow <andrew@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0371
  Status: Draft
  Type: Standards Track
  Created: 2021-06-21
  License: BSD-2-Clause
```

## Introduction

### Abstract

This document proposes additional fields for BIP 174 PSBTv0 and BIP 370
PSBTv2 that allow for BIP 340/341/342 Taproot data to be included in a
PSBT of any version. These will be fields for signatures and scripts
that are relevant to the creation of Taproot inputs.

### Copyright

This BIP is licensed under the 2-clause BSD license.

### Motivation

BIPs 340, 341, and 342 specify Taproot which provides a wholly new way
to create and spend Bitcoin outputs. The existing PSBT fields are unable
to support Taproot due to the new signature algorithm and the method by
which scripts are embedded inside of a Taproot output. Therefore new
fields must be defined to allow PSBTs to carry the information necessary
for signing Taproot inputs.

## Specification

The new per-input types are defined as follows:

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
<td><p>Taproot Key Spend Signature</p></td>
<td><p><code>PSBT_IN_TAP_KEY_SIG = 0x13</code></p></td>
<td><p>None</p></td>
<td><p>No key data [1]</p></td>
<td><p><signature></p></td>
<td><p>The 64 or 65 byte Schnorr signature for key path spending a Taproot output. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="even">
<td><p>Taproot Script Spend Signature</p></td>
<td><p><code>PSBT_IN_TAP_SCRIPT_SIG = 0x14</code></p></td>
<td><p><xonlypubkey><code> </code><leafhash></p></td>
<td><p>A 32 byte X-only public key involved in a leaf script concatenated with the 32 byte hash of the leaf it is part of.</p></td>
<td><p><signature></p></td>
<td><p>The 64 or 65 byte Schnorr signature for this pubkey and leaf combination. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="odd">
<td><p>Taproot Leaf Script</p></td>
<td><p><code>PSBT_IN_TAP_LEAF_SCRIPT = 0x15</code></p></td>
<td><p><control block></p></td>
<td><p>The control block for this leaf as specified in BIP 341. The control block contains the merkle tree path to this leaf.</p></td>
<td><p><tt></p>
<script>
<p>&lt;8-bit uint&gt;</tt></p></td>
<td><p>The script for this leaf as would be provided in the witness stack followed by the single byte leaf version. Note that the leaves included in this field should be those that the signers of this input are expected to be able to sign for. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="even">
<td><p>Taproot Key BIP 32 Derivation Path</p></td>
<td><p><code>PSBT_IN_TAP_BIP32_DERIVATION = 0x16</code></p></td>
<td><p><xonlypubkey></p></td>
<td><p>A 32 byte X-only public key involved in this input. It may be the internal key, or a key present in a leaf script.</p></td>
<td><p><hashes len><code> </code><leaf hash><code>* &lt;4 byte fingerprint&gt; &lt;32-bit uint&gt;*</code></p></td>
<td><p>A compact size unsigned integer representing the number of leaf hashes, followed by a list of leaf hashes, followed by the 4 byte master key fingerprint concatenated with the derivation path of the public key. The derivation path is represented as 32-bit little endian unsigned integer indexes concatenated with each other. Public keys are those needed to spend this output. The leaf hashes are of the leaves which involve this public key. The internal key does not have leaf hashes, so can be indicated with a <code>hashes len</code> of 0. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="odd">
<td><p>Taproot Internal Key</p></td>
<td><p><code>PSBT_IN_TAP_INTERNAL_KEY = 0x17</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><pubkey></p></td>
<td><p>The X-only pubkey used as the internal key in this output.[2] Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="even">
<td><p>Taproot Merkle Root</p></td>
<td><p><code>PSBT_IN_TAP_MERKLE_ROOT = 0x18</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><pubkey></p></td>
<td><p>The 32 byte Merkle root hash. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
</tbody>
</table>

The new per-output types are defined as follows:

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
<td><p>Taproot Internal Key</p></td>
<td><p><code>PSBT_OUT_TAP_INTERNAL_KEY = 0x05</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><pubkey></p></td>
<td><p>The X-only pubkey used as the internal key in this output.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="even">
<td><p>Taproot Tree</p></td>
<td><p><code>PSBT_OUT_TAP_TREE = 0x06</code></p></td>
<td><p>None</p></td>
<td><p>No key data</p></td>
<td><p><tt>{&lt;8-bit uint depth&gt; &lt;8-bit uint leaf version&gt; <scriptlen></p>
<script>
<p>}*</tt></p></td>
<td><p>One or more tuples representing the depth, leaf version, and script for a leaf in the Taproot tree, allowing the entire tree to be reconstructed. The tuples must be in depth first search order so that the tree is correctly reconstructed. Each tuple is an 8-bit unsigned integer representing the depth in the Taproot tree for this script, an 8-bit unsigned integer representing the leaf version, the length of the script as a compact size unsigned integer, and the script itself.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
<tr class="odd">
<td><p>Taproot Key BIP 32 Derivation Path</p></td>
<td><p><code>PSBT_OUT_TAP_BIP32_DERIVATION = 0x07</code></p></td>
<td><p><xonlypubkey></p></td>
<td><p>A 32 byte X-only public key involved in this output. It may be the internal key, or a key present in a leaf script.</p></td>
<td><p><hashes len><code> </code><leaf hash><code>* &lt;4 byte fingerprint&gt; &lt;32-bit uint&gt;*</code></p></td>
<td><p>A compact size unsigned integer representing the number of leaf hashes, followed by a list of leaf hashes, followed by the 4 byte master key fingerprint concatenated with the derivation path of the public key. The derivation path is represented as 32-bit little endian unsigned integer indexes concatenated with each other. Public keys are those needed to spend this output. The leaf hashes are of the leaves which involve this public key. The internal key does not have leaf hashes, so can be indicated with a <code>hashes len</code> of 0. Finalizers should remove this field after <code>PSBT_IN_FINAL_SCRIPTWITNESS</code> is constructed.</p></td>
<td></td>
<td></td>
<td><p>0, 2</p></td>
</tr>
</tbody>
</table>

### UTXO Types

BIP 174 recommends using `PSBT_IN_NON_WITNESS_UTXO` for all inputs
because of potential attacks involving an updater lying about the
amounts in an output. Because a Taproot signature will commit to all of
the amounts and output scripts spent by the inputs of the transaction,
such attacks are prevented as any such lying would result in an invalid
signature. Thus Taproot inputs can use just `PSBT_IN_WITNESS_UTXO`.

## Compatibility

These are simply new fields added to the existing PSBT format. Because
PSBT is designed to be extensible, old software will ignore the new
fields.

## Test Vectors

TBD

## Rationale

<references/>

## Reference implementation

The reference implementation of the PSBT format is available at TBD.

## Acknowledgements

TBD

1.  **Why is there no key data for `PSBT_IN_TAP_KEY_SIG`**The signature
    in a key path spend corresponds directly with the pubkey provided in
    the output script. Thus it is not necessary to provide any metadata
    that attaches the key path spend signature to a particular pubkey.
2.  **Why is the internal key provided?**The internal key is not
    necessarily the same key as in the Taproot output script. BIP 341
    recommends tweaking the key with the hash of itself. It may be
    necessary for signers to know what the internal key actually is so
    that they are able to determine whether an input can be signed by
    it.
