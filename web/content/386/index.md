
+++
title = "tr() Output Script Descriptors"
date = 2021-06-27
weight = 386

[taxonomies]
authors = ["Pieter Wuille", "Ava Chow"]
status = ["Final"]

[extra]
bip = 386
status = ["Final"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0386.mediawiki"
note = "THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING"
+++

```
  BIP: 386
  Layer: Applications
  Title: tr() Output Script Descriptors
  Author: Pieter Wuille <pieter@wuille.net>
          Ava Chow <me@achow101.com>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0386
  Status: Final
  Type: Informational
  Created: 2021-06-27
  License: BSD-2-Clause
  Requires: 380
```

<h2>Abstract</h2>


This document specifies <tt>tr()</tt> output script descriptors.
<tt>tr()</tt> descriptors take a key and optionally a tree of scripts and produces a P2TR output script.

<h2>Copyright</h2>


This BIP is licensed under the BSD 2-clause license.

<h2>Motivation</h2>


Taproot added one additional standard output script format: P2TR.
These expressions allow specifying those formats as a descriptor.

<h2>Specification</h2>


A new script expression is defined: <tt>tr()</tt>.
A new expression is defined: Tree Expressions

<h3>Tree Expression</h3>


A Tree Expression (denoted <tt>TREE</tt>) is an expression which represents a tree of scripts.
The way the tree is represented in an output script is dependent on the higher level expressions.

A Tree Expression is:
*  Any Script Expression that is allowed at the level this Tree Expression is in.
*  A pair of Tree Expressions consisting of:
    *  An open brace <tt>{</tt>
    *  A Tree Expression
    *  A comma <tt>,</tt>
    *  A Tree Expression
    *  A closing brace <tt>}</tt>


<h3><tt>tr()</tt></h3>


The <tt>tr(KEY)</tt> or <tt>tr(KEY, TREE)</tt> expression can only be used as a top level expression.
All key expressions under any <tt>tr()</tt> expression must create x-only public keys.

<tt>tr(KEY)</tt> takes a single key expression as an argument and produces a P2TR output script which does not have a script path.
Each key produced by the key expression is used as the internal key of a P2TR output as specified by <a href="/341" target="_blank">BIP 341</a>.
Specifically, "If the spending conditions do not require a script path, the output key should commit to an unspendable script path instead of having no script path.
This can be achieved by computing the output key point as _Q = P + int(hash<sub>TapTweak</sub>(bytes(P)))G_."

```
internal_key:       lift_x(KEY)
32_byte_output_key: internal_key + int(HashTapTweak(bytes(internal_key)))G
scriptPubKey:       OP_1 <32_byte_output_key>
```

<tt>tr(KEY, TREE)</tt> takes a key expression as the first argument, and a tree expression as the second argument and produces a P2TR output script which has a script path.
The keys produced by the first key expression are used as the internal key as specified by <a href="/341" target="_blank">BIP 341</a>.
The Tree expression becomes the Taproot script tree as described in BIP 341.
A merkle root is computed from this tree and combined with the internal key to create the Taproot output key.

```
internal_key:       lift_x(KEY)
merkle_root:        HashTapBranch(TREE)
32_byte_output_key: internal_key + int(HashTapTweak(bytes(internal_key) || merkle_root))G
scriptPubKey:       OP_1 <32_byte_output_key>
```

<h3>Modified Key Expression</h3>


Key Expressions within a <tt>tr()</tt> expression must only create x-only public keys.
Uncompressed public keys are not allowed, but compressed public keys would be implicitly converted to x-only public keys.
The keys derived from extended keys must be serialized as x-only public keys.
An additional key expression is defined only for use within a <tt>tr()</tt> descriptor:

*  A 64 hex character string representing an x-only public key


<h2>Test Vectors</h2>


Valid descriptors followed by the scripts they produce. Descriptors involving derived child keys will have the 0th, 1st, and 2nd scripts listed.

*  <tt>tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd)</tt>
    *  <tt>512077aab6e066f8a7419c5ab714c12c67d25007ed55a43cadcacb4d7a970a093f11</tt>
*  <tt>tr(L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)</tt>
    *  <tt>512077aab6e066f8a7419c5ab714c12c67d25007ed55a43cadcacb4d7a970a093f11</tt>
*  <tt>tr(xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/0/*,pk(xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc/1/*))</tt>
    *  <tt>512078bc707124daa551b65af74de2ec128b7525e10f374dc67b64e00ce0ab8b3e12</tt>
    *  <tt>512001f0a02a17808c20134b78faab80ef93ffba82261ccef0a2314f5d62b6438f11</tt>
    *  <tt>512021024954fcec88237a9386fce80ef2ced5f1e91b422b26c59ccfc174c8d1ad25</tt>
*  <tt>tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd,pk(669b8afcec803a0d323e9a17f3ea8e68e8abe5a278020a929adbec52421adbd0))</tt>
    *  <tt>512017cf18db381d836d8923b1bdb246cfcd818da1a9f0e6e7907f187f0b2f937754</tt>
*  <tt>tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd,{pk(xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334/0),{{pk(xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL),pk(02df12b7035bdac8e3bab862a3a83d06ea6b17b6753d52edecba9be46f5d09e076)},pk(L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)}})</tt>
    *  <tt>512071fff39599a7b78bc02623cbe814efebf1a404f5d8ad34ea80f213bd8943f574</tt>


Invalid Descriptors

*  Uncompressed private key: <tt>tr(5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss)</tt>
*  Uncompressed public key: <tt>tr(04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235)</tt>
*  <tt>tr()</tt> nested in <tt>wsh</tt>: <tt>wsh(tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd))</tt>
*  <tt>tr()</tt> nested in <tt>sh</tt>: <tt>sh(tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd))</tt>
*  <tt>pkh()</tt> nested in <tt>tr</tt>: <tt>tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd, pkh(L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1))</tt>


<h2>Backwards Compatibility</h2>


<tt>tr()</tt> descriptors use the format and general operation specified in <a href="/380" target="_blank">380</a>.
As these are a set of wholly new descriptors, they are not compatible with any implementation.
However the scripts produced are standard scripts so existing software are likely to be familiar with them.

Tree Expressions are largely incompatible with existing script expressions due to the restrictions in those expressions.
As of 2021-06-27, the only allowed script expression that can be used in a tree expression is <tt>pk()</tt>.
However there will be future BIPs that specify script expressions that can be used in tree expressions.

<h2>Reference Implementation</h2>


<tt>tr()</tt> descriptors have been implemented in Bitcoin Core since version 22.0.
