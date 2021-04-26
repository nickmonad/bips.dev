+++
title = "Validation of Taproot Scripts"
date = 2020-01-19
weight = 342
in_search_index = true

[taxonomies]
authors = ["Pieter Wuille", "Jonas Nick", "Anthony Towns"]
status = ["Draft"]

[extra]
bip = 342
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki"
+++

``` 
  BIP: 342
  Layer: Consensus (soft fork)
  Title: Validation of Taproot Scripts
  Author: Pieter Wuille <pieter.wuille@gmail.com>
          Jonas Nick <jonasd.nick@gmail.com>
          Anthony Towns <aj@erisian.com.au>
  Comments-Summary: No comments yet.
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0342
  Status: Draft
  Type: Standards Track
  Created: 2020-01-19
  License: BSD-3-Clause
  Requires: 340, 341
  Post-History: 2019-05-06: https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2019-May/016914.html [bitcoin-dev] Taproot proposal
```

## Introduction

### Abstract

This document specifies the semantics of the initial scripting system
under [BIP341](bip-0341.mediawiki "wikilink").

### Copyright

This document is licensed under the 3-clause BSD license.

### Motivation

[BIP341](bip-0341.mediawiki "wikilink") proposes improvements to just
the script structure, but some of its goals are incompatible with the
semantics of certain opcodes within the scripting language itself. While
it is possible to deal with these in separate optional improvements,
their impact is not guaranteed unless they are addressed simultaneously
with [BIP341](bip-0341.mediawiki "wikilink") itself.

Specifically, the goal is making **Schnorr signatures**, **batch
validation**, and **signature hash** improvements available to spends
that use the script system as well.

## Design

In order to achieve these goals, signature opcodes `OP_CHECKSIG` and
`OP_CHECKSIGVERIFY` are modified to verify Schnorr signatures as
specified in [BIP340](bip-0340.mediawiki "wikilink") and to use a
signature message algorithm based on the common message calculation in
[BIP341](bip-0341.mediawiki "wikilink"). The tapscript signature message
also simplifies `OP_CODESEPARATOR` handling and makes it more efficient.

The inefficient `OP_CHECKMULTISIG` and `OP_CHECKMULTISIGVERIFY` opcodes
are disabled. Instead, a new opcode `OP_CHECKSIGADD` is introduced to
allow creating the same multisignature policies in a batch-verifiable
way. Tapscript uses a new, simpler signature opcode limit fixing
complicated interactions with transaction weight. Furthermore, a
potential malleability vector is eliminated by requiring MINIMALIF.

Tapscript can be upgraded through soft forks by defining unknown key
types, for example to add new `hash_types` or signature algorithms.
Additionally, the new tapscript `OP_SUCCESS` opcodes allow introducing
new opcodes more cleanly than through `OP_NOP`.

## Specification

The rules below only apply when validating a transaction input for which
all of the conditions below are true:

  - The transaction input is a **segregated witness spend** (i.e., the
    scriptPubKey contains a witness program as defined in
    [BIP141](bip-0141.mediawiki "wikilink")).
  - It is a **taproot spend** as defined in
    [BIP341](bip-0341.mediawiki#design "wikilink") (i.e., the witness
    version is 1, the witness program is 32 bytes, and it is not P2SH
    wrapped).
  - It is a **script path spend** as defined in
    [BIP341](bip-0341.mediawiki#design "wikilink") (i.e., after removing
    the optional annex from the witness stack, two or more stack
    elements remain).
  - The leaf version is *0xc0* (i.e. the first byte of the last witness
    element after removing the optional annex is *0xc0* or *0xc1*),
    marking it as a **tapscript spend**.

Validation of such inputs must be equivalent to performing the following
steps in the specified order.

1.  If the input is invalid due to BIP141 or BIP341, fail.
2.  The script as defined in BIP341 (i.e., the penultimate witness stack
    element after removing the optional annex) is called the
    **tapscript** and is decoded into opcodes, one by one:
    1.  If any opcode numbered *80, 98, 126-129, 131-134, 137-138,
        141-142, 149-153, 187-254* is encountered, validation succeeds
        (none of the rules below apply). This is true even if later
        bytes in the tapscript would fail to decode otherwise. These
        opcodes are renamed to `OP_SUCCESS80`, ..., `OP_SUCCESS254`, and
        collectively known as `OP_SUCCESSx`<ref>**`OP_SUCCESSx`**
        `OP_SUCCESSx` is a mechanism to upgrade the Script system. Using
        an `OP_SUCCESSx` before its meaning is defined by a softfork is
        insecure and leads to fund loss. The inclusion of `OP_SUCCESSx`
        in a script will pass it unconditionally. It precedes any script
        execution rules to avoid the difficulties in specifying various
        edge cases, for example: `OP_SUCCESSx` in a script with an input
        stack larger than 1000 elements, `OP_SUCCESSx` after too many
        signature opcodes, or even scripts with conditionals lacking
        `OP_ENDIF`. The mere existence of an `OP_SUCCESSx` anywhere in
        the script will guarantee a pass for all such cases.
        `OP_SUCCESSx` are similar to the `OP_RETURN` in very early
        bitcoin versions (v0.1 up to and including v0.3.5). The original
        `OP_RETURN` terminates script execution immediately, and return
        pass or fail based on the top stack element at the moment of
        termination. This was one of a major design flaws in the
        original bitcoin protocol as it permitted unconditional third
        party theft by placing an `OP_RETURN` in `scriptSig`. This is
        not a concern in the present proposal since it is not possible
        for a third party to inject an `OP_SUCCESSx` to the validation
        process, as the `OP_SUCCESSx` is part of the script (and thus
        committed to by the taproot output), implying the consent of the
        coin owner. `OP_SUCCESSx` can be used for a variety of upgrade
        possibilities:

<!-- end list -->

  - An `OP_SUCCESSx` could be turned into a functional opcode through a
    softfork. Unlike `OP_NOPx`-derived opcodes which only have read-only
    access to the stack, `OP_SUCCESSx` may also write to the stack. Any
    rule changes to an `OP_SUCCESSx`-containing script may only turn a
    valid script into an invalid one, and this is always achievable with
    softforks.
  - Since `OP_SUCCESSx` precedes size check of initial stack and push
    opcodes, an `OP_SUCCESSx`-derived opcode requiring stack elements
    bigger than 520 bytes may uplift the limit in a softfork.
  - `OP_SUCCESSx` may also redefine the behavior of existing opcodes so
    they could work together with the new opcode. For example, if an
    `OP_SUCCESSx`-derived opcode works with 64-bit integers, it may also
    allow the existing arithmetic opcodes in the *same script* to do the
    same.
  - Given that `OP_SUCCESSx` even causes potentially unparseable scripts
    to pass, it can be used to introduce multi-byte opcodes, or even a
    completely new scripting language when prefixed with a specific
    `OP_SUCCESSx` opcode.</ref>.

<!-- end list -->

1.  1.  If any push opcode fails to decode because it would extend past
        the end of the tapscript, fail.

2.  If the **initial stack** as defined in BIP341 (i.e., the witness
    stack after removing both the optional annex and the two last stack
    elements after that) violates any resource limits (stack size, and
    size of the elements in the stack; see "Resource Limits" below),
    fail. Note that this check can be bypassed using `OP_SUCCESSx`.

3.  The tapscript is executed according to the rules in the following
    section, with the initial stack as input.
    
    1.  If execution fails for any reason, fail.
    2.  If the execution results in anything but exactly one element on
        the stack which evaluates to true with `CastToBool()`, fail.

4.  If this step is reached without encountering a failure, validation
    succeeds.

### Script execution

The execution rules for tapscript are based on those for P2WSH according
to BIP141, including the `OP_CHECKLOCKTIMEVERIFY` and
`OP_CHECKSEQUENCEVERIFY` opcodes defined in
[BIP65](bip-0065.mediawiki "wikilink") and
[BIP112](bip-0112.mediawiki "wikilink"), but with the following
modifications:

  - **Disabled script opcodes** The following script opcodes are
    disabled in tapscript: `OP_CHECKMULTISIG` and
    `OP_CHECKMULTISIGVERIFY`\[1\]. The disabled opcodes behave in the
    same way as `OP_RETURN`, by failing and terminating the script
    immediately when executed, and being ignored when found in
    unexecuted branch of the script.
  - **Consensus-enforced MINIMALIF** The MINIMALIF rules, which are only
    a standardness rule in P2WSH, are consensus enforced in tapscript.
    This means that the input argument to the `OP_IF` and `OP_NOTIF`
    opcodes must be either exactly 0 (the empty vector) or exactly 1
    (the one-byte vector with value 1)\[2\].
  - **OP\_SUCCESSx opcodes** As listed above, some opcodes are renamed
    to `OP_SUCCESSx`, and make the script unconditionally valid.
  - **Signature opcodes**. The `OP_CHECKSIG` and `OP_CHECKSIGVERIFY` are
    modified to operate on Schnorr public keys and signatures (see
    [BIP340](bip-0340.mediawiki "wikilink")) instead of ECDSA, and a new
    opcode `OP_CHECKSIGADD` is added.
      - The opcode 186 (`0xba`) is named as `OP_CHECKSIGADD`.
        \[3\]<ref>**Alternatives to `CHECKMULTISIG`** There are multiple
        ways of implementing a threshold *k*-of-*n* policy using Taproot
        and Tapscript:
  - **Using a single `OP_CHECKSIGADD`-based script** A `CHECKMULTISIG`
    script ` m  `<pubkey_1>`  ...  `<pubkey_n>`  n CHECKMULTISIG ` with
    witness ` 0  `<signature_1>`  ...  `<signature_m> can be rewritten
    as script <pubkey_1>`  CHECKSIG  `<pubkey_2>`  CHECKSIGADD ...
     `<pubkey_n>`  CHECKSIGADD m NUMEQUAL ` with witness <w_n>`  ...
     `<w_1>. Every witness element `w_i` is either a signature
    corresponding to `pubkey_i` or an empty vector. A similar
    `CHECKMULTISIGVERIFY` script can be translated to BIP342 by
    replacing `NUMEQUAL` with `NUMEQUALVERIFY`. This approach has very
    similar characteristics to the existing `OP_CHECKMULTISIG`-based
    scripts.
  - **Using a *k*-of-*k* script for every combination** A *k*-of-*n*
    policy can be implemented by splitting the script into several
    leaves of the Merkle tree, each implementing a *k*-of-*k* policy
    using <pubkey_1>`  CHECKSIGVERIFY ... <pubkey_(n-1)> CHECKSIGVERIFY
     `<pubkey_n>`  CHECKSIG `. This may be preferable for privacy
    reasons over the previous approach, as it only exposes the
    participating public keys, but it is only more cost effective for
    small values of *k* (1-of-*n* for any *n*, 2-of-*n* for *n ≥ 6*,
    3-of-*n* for *n ≥ 9*, ...). Furthermore, the signatures here commit
    to the branch used, which means signers need to be aware of which
    other signers will be participating, or produce signatures for each
    of the tree leaves.
  - **Using an aggregated public key for every combination** Instead of
    building a tree where every leaf consists of *k* public keys, it is
    possible instead build a tree where every leaf contains a single
    *aggregate* of those *k* keys using
    [MuSig](https://eprint.iacr.org/2018/068). This approach is far more
    efficient, but does require a 3-round interactive signing protocol
    to jointly produce the (single) signature.
  - **Native Schnorr threshold signatures** Multisig policies can also
    be realized with [threshold
    signatures](http://cacr.uwaterloo.ca/techreports/2001/corr2001-13.ps)
    using verifiable secret sharing. This results in outputs and inputs
    that are indistinguishable from single-key payments, but at the cost
    of needing an interactive protocol (and associated backup
    procedures) before determining the address to send to.</ref>

### Rules for signature opcodes

The following rules apply to `OP_CHECKSIG`, `OP_CHECKSIGVERIFY`, and
`OP_CHECKSIGADD`.

  - For `OP_CHECKSIGVERIFY` and `OP_CHECKSIG`, the public key (top
    element) and a signature (second to top element) are popped from the
    stack.
      - If fewer than 2 elements are on the stack, the script MUST fail
        and terminate immediately.
  - For `OP_CHECKSIGADD`, the public key (top element), a `CScriptNum`
    `n` (second to top element), and a signature (third to top element)
    are popped from the stack.
      - If fewer than 3 elements are on the stack, the script MUST fail
        and terminate immediately.
      - If `n` is larger than 4 bytes, the script MUST fail and
        terminate immediately.
  - If the public key size is zero, the script MUST fail and terminate
    immediately.
  - If the public key size is 32 bytes, it is considered to be a public
    key as described in BIP340:
      - If the signature is not the empty vector, the signature is
        validated against the public key (see the next subsection).
        Validation failure in this case immediately terminates script
        execution with failure.
  - If the public key size is not zero and not 32 bytes, the public key
    is of an *unknown public key type*\[4\] and no actual signature
    verification is applied. During script execution of signature
    opcodes they behave exactly as known public key types except that
    signature validation is considered to be successful.
  - If the script did not fail and terminate before this step,
    regardless of the public key type:
      - If the signature is the empty vector:
          - For `OP_CHECKSIGVERIFY`, the script MUST fail and terminate
            immediately.
          - For `OP_CHECKSIG`, an empty vector is pushed onto the stack,
            and execution continues with the next opcode.
          - For `OP_CHECKSIGADD`, a `CScriptNum` with value `n` is
            pushed onto the stack, and execution continues with the next
            opcode.
      - If the signature is not the empty vector, the opcode is counted
        towards the sigops budget (see further).
          - For `OP_CHECKSIGVERIFY`, execution continues without any
            further changes to the stack.
          - For `OP_CHECKSIG`, a 1-byte value `0x01` is pushed onto the
            stack.
          - For `OP_CHECKSIGADD`, a `CScriptNum` with value of `n + 1`
            is pushed onto the stack.

### Signature validation

To validate a signature *sig* with public key *p*:

  - Compute the tapscript message extension *ext*, consisting of the
    concatenation of:
      - *tapleaf\_hash* (32): the tapleaf hash as defined in
        [BIP341](bip-0341.mediawiki#design "wikilink")
      - *key\_version* (1): a constant value *0x00* representing the
        current version of public keys in the tapscript signature opcode
        execution.
      - *codesep\_pos* (4): the opcode position of the last executed
        `OP_CODESEPARATOR` before the currently executed signature
        opcode, with the value in little endian (or *0xffffffff* if none
        executed). The first opcode in a script has a position of 0. A
        multi-byte push opcode is counted as one opcode, regardless of
        the size of data being pushed. Opcodes in parsed but unexecuted
        branches count towards this value as well.
  - If the *sig* is 64 bytes long, return *Verify(p,
    hash<sub>TapSighash</sub>(0x00 || SigMsg(0x00, 1) || ext), sig)*,
    where *Verify* is defined in
    [BIP340](bip-0340.mediawiki#design "wikilink").
  - If the *sig* is 65 bytes long, return *sig\[64\] ≠ 0x00 and
    Verify(p, hash<sub>TapSighash</sub>(0x00 || SigMsg(sig\[64\], 1) ||
    ext), sig\[0:64\])*.
  - Otherwise, fail.

In summary, the semantics of signature validation is identical to
BIP340, except the following:

1.  The signature message includes the tapscript-specific data
    *key\_version*.\[5\]
2.  The signature message commits to the executed script through the
    *tapleaf\_hash* which includes the leaf version and script instead
    of *scriptCode*. This implies that this commitment is unaffected by
    `OP_CODESEPARATOR`.
3.  The signature message includes the opcode position of the last
    executed `OP_CODESEPARATOR`.\[6\]

### Resource limits

In addition to changing the semantics of a number of opcodes, there are
also some changes to the resource limitations:

  - **Script size limit** The maximum script size of 10000 bytes does
    not apply. Their size is only implicitly bounded by the block weight
    limit.\[7\]
  - **Non-push opcodes limit** The maximum non-push opcodes limit of 201
    per script does not apply.\[8\]
  - **Sigops limit** The sigops in tapscripts do not count towards the
    block-wide limit of 80000 (weighted). Instead, there is a per-script
    sigops *budget*. The budget equals 50 + the total serialized size in
    bytes of the transaction input's witness (including the
    `CompactSize` prefix). Executing a signature opcode (`OP_CHECKSIG`,
    `OP_CHECKSIGVERIFY`, or `OP_CHECKSIGADD`) with a non-empty signature
    decrements the budget by 50. If that brings the budget below zero,
    the script fails immediately. Signature opcodes with unknown public
    key type and non-empty signature are also counted.\[9\]\[10\]\[11\].
  - **Stack + altstack element count limit** The existing limit of 1000
    elements in the stack and altstack together after every executed
    opcode remains. It is extended to also apply to the size of initial
    stack.
  - **Stack element size limit** The existing limit of maximum 520 bytes
    per stack element remains, both in the initial stack and in push
    opcodes.

## Rationale

<references />

## Deployment

This proposal is deployed identically to Taproot
([BIP341](bip-0341.mediawiki "wikilink")).

## Examples

The Taproot ([BIP341](bip-0341.mediawiki "wikilink")) test vectors also
contain examples for Tapscript execution.

## Acknowledgements

This document is the result of many discussions and contains
contributions by a number of people. The authors wish to thank all those
who provided valuable feedback and reviews, including the participants
of the [structured reviews](https://github.com/ajtowns/taproot-review).

1.  **Why are `OP_CHECKMULTISIG` and `OP_CHECKMULTISIGVERIFY` disabled,
    and not turned into OP\_SUCCESSx?** This is a precaution to make
    sure people who accidentally keep using `OP_CHECKMULTISIG` in
    Tapscript notice a problem immediately. It also avoids the
    complication of script disassemblers needing to become
    context-dependent.
2.  **Why make MINIMALIF consensus?** This makes it considerably easier
    to write non-malleable scripts that take branch information from the
    stack.
3.  **`OP_CHECKSIGADD`** This opcode is added to compensate for the loss
    of `OP_CHECKMULTISIG`-like opcodes, which are incompatible with
    batch verification. `OP_CHECKSIGADD` is functionally equivalent to
    `OP_ROT OP_SWAP OP_CHECKSIG OP_ADD`, but only takes 1 byte. All
    `CScriptNum`-related behaviours of `OP_ADD` are also applicable to
    `OP_CHECKSIGADD`.
4.  **Unknown public key types** allow adding new signature validation
    rules through softforks. A softfork could add actual signature
    validation which either passes or makes the script fail and
    terminate immediately. This way, new `SIGHASH` modes can be added,
    as well as [NOINPUT-tagged public
    keys](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2018-December/016549.html)
    and a public key constant which is replaced by the taproot internal
    key for signature validation.
5.  **Why does the signature message commit to the *key\_version*?**
    This is for future extensions that define unknown public key types,
    making sure signatures can't be moved from one key type to another.
6.  **Why does the signature message include the position of the last
    executed `OP_CODESEPARATOR`?** This allows continuing to use
    `OP_CODESEPARATOR` to sign the executed path of the script. Because
    the `codeseparator_position` is the last input to the hash, the
    SHA256 midstate can be efficiently cached for multiple
    `OP_CODESEPARATOR`s in a single script. In contrast, the BIP143
    handling of `OP_CODESEPARATOR` is to commit to the executed script
    only from the last executed `OP_CODESEPARATOR` onwards which
    requires unnecessary rehashing of the script. It should be noted
    that the one known `OP_CODESEPARATOR` use case of saving a second
    public key push in a script by sharing the first one between two
    code branches can be most likely expressed even cheaper by moving
    each branch into a separate taproot leaf.
7.  **Why is a limit on script size no longer needed?** Since there is
    no `scriptCode` directly included in the signature hash (only
    indirectly through a precomputable tapleaf hash), the CPU time spent
    on a signature check is no longer proportional to the size of the
    script being executed.
8.  **Why is a limit on the number of opcodes no longer needed?** An
    opcode limit only helps to the extent that it can prevent data
    structures from growing unboundedly during execution (both because
    of memory usage, and because of time that may grow in proportion to
    the size of those structures). The size of stack and altstack is
    already independently limited. By using O(1) logic for `OP_IF`,
    `OP_NOTIF`, `OP_ELSE`, and `OP_ENDIF` as suggested
    [here](https://bitslog.com/2017/04/17/new-quadratic-delays-in-bitcoin-scripts/)
    and implemented
    [here](https://github.com/bitcoin/bitcoin/pull/16902), the only
    other instance can be avoided as well.
9.  **The tapscript sigop limit** The signature opcode limit protects
    against scripts which are slow to verify due to excessively many
    signature operations. In tapscript the number of signature opcodes
    does not count towards the BIP141 or legacy sigop limit. The old
    sigop limit makes transaction selection in block construction
    unnecessarily difficult because it is a second constraint in
    addition to weight. Instead, the number of tapscript signature
    opcodes is limited by witness weight. Additionally, the limit
    applies to the transaction input instead of the block and only
    actually executed signature opcodes are counted. Tapscript execution
    allows one signature opcode per 50 witness weight units plus one
    free signature opcode.
10. **Parameter choice of the sigop limit** Regular witnesses are
    unaffected by the limit as their weight is composed of public key
    and (`SIGHASH_ALL`) signature pairs with *33 + 65* weight units each
    (which includes a 1 weight unit `CompactSize` tag). This is also the
    case if public keys are reused in the script because a signature's
    weight alone is 65 or 66 weight units. However, the limit increases
    the fees of abnormal scripts with duplicate signatures (and public
    keys) by requiring additional weight. The weight per sigop factor 50
    corresponds to the ratio of BIP141 block limits: 4 mega weight units
    divided by 80,000 sigops. The "free" signature opcode permitted by
    the limit exists to account for the weight of the non-witness parts
    of the transaction input.
11. **Why are only signature opcodes counted toward the budget, and not
    for example hashing opcodes or other expensive operations?** It
    turns out that the CPU cost per witness byte for verification of a
    script consisting of the maximum density of signature checking
    opcodes (taking the 50 WU/sigop limit into account) is already very
    close to that of scripts packed with other opcodes, including
    hashing opcodes (taking the 520 byte stack element limit into
    account) and `OP_ROLL` (taking the 1000 stack element limit into
    account). That said, the construction is very flexible, and allows
    adding new signature opcodes like `CHECKSIGFROMSTACK` to count
    towards the limit through a soft fork. Even if in the future new
    opcodes are introduced which change normal script cost there is no
    need to stuff the witness with meaningless data. Instead, the
    taproot annex can be used to add weight to the witness without
    increasing the actual witness size.
