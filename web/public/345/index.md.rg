+++
title = "OP_VAULT"
date = 2023-02-03
weight = 345
in_search_index = true

[taxonomies]
authors = ["James O'Beirne", "Greg Sanders", "Anthony Towns"]
status = ["Draft"]

[extra]
bip = 345
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0345.mediawiki"
+++

``` 
  BIP: 345
  Layer: Consensus (soft fork)
  Title: OP_VAULT
  Author: James O'Beirne <vaults@au92.org>
          Greg Sanders <gsanders87@gmail.com>
          Anthony Towns <aj@erisian.com.au>
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0345
  Status: Draft
  Type: Standards Track
  Created: 2023-02-03
  License: BSD-3-Clause
  Post-History: 2023-01-09: https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2023-January/021318.html [bitcoin-dev] OP_VAULT announcment
                2023-03-01: https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2023-March/021510.html [bitcoin-dev] BIP for OP_VAULT
```

## Introduction

This BIP proposes two new tapscript opcodes that add consensus support
for a specialized covenant: `OP_VAULT` and `OP_VAULT_RECOVER`. These
opcodes, in conjunction with `OP_CHECKTEMPLATEVERIFY`
([BIP-0119](https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki)),
allow users to enforce a delay period before designated coins may be
spent to an arbitrary destination, with the exception of a prespecified
"recovery" path. At any time prior to final withdrawal, the coins can be
spent to the recovery path.

### Copyright

This document is licensed under the 3-clause BSD license.

### Motivation

The hazard of custodying Bitcoin is well-known. Users of Bitcoin must go
to significant effort to secure their private keys, and hope that once
provisioned their custody system does not yield to any number of
evolving and persistent threats. Users have little means to intervene
once a compromise is detected. This proposal introduces a mechanism that
significantly mitigates the worst-case outcome of key compromise: coin
loss.

Introducing a way to intervene during unexpected spends allows users to
incorporate highly secure key storage methods or unusual fallback
strategies that are only exercised in the worst case, and which may
otherwise be operationally prohibitive. The goal of this proposal is to
make this strategy usable for custodians of any size with minimal
complication.

#### Example uses

A common configuration for an individual custodying Bitcoin is "single
signature and passphrase" using a hardware wallet. A user with such a
configuration might be concerned about the risk associated with relying
on a single manufacturer for key management, as well as physical access
to the hardware.

This individual can use `OP_VAULT` to make use of a highly secure key as
the unlikely recovery path, while using their existing signing procedure
as the withdrawal trigger key with a configured spend delay of e.g. 1
day.

The recovery path key can be of a highly secure nature that might
otherwise make it impractical for daily use. For example, the key could
be generated in some analog fashion, or on an old computer that is then
destroyed, with the private key replicated only in paper form. Or the
key could be a 2-of-3 multisig using devices from different
manufacturers. Perhaps the key is geographically or socially
distributed.

Since it can be any Bitcoin script policy, the recovery key can include
a number of spending conditions, e.g. a time-delayed fallback to an
"easier" recovery method, in case the highly secure key winds up being
*too* highly secure.

The user can run software on their mobile device that monitors the
blockchain for spends of the vault outpoints. If the vaulted coins move
in an unexpected way, the user can immediately sweep them to the
recovery path, but spending the coins on a daily basis works in the same
way it did prior to vaulting (aside from the spend delay).

Institutional custodians of Bitcoin may use vaults in similar fashion.

##### Provable timelocks

This proposal provides a mitigation to the ["$5 wrench
attack."](https://web.archive.org/web/20230210123933/https://xkcd.com/538/)
By setting the spend delay to, say, a week, and using as the recovery
path a script that enforces a longer relative timelock, the owner of the
vault can prove that he is unable to access its value immediately. To
the author's knowledge, this is the only way to configure this defense
without rolling timelocked coins for perpetuity or relying on a trusted
third party.

## Goals

![bip-0345/vaults-Basic.png](bip-0345/vaults-Basic.png
"bip-0345/vaults-Basic.png")

Vaults in Bitcoin have been discussed formally since 2016
([MES16](http://fc16.ifca.ai/bitcoin/papers/MES16.pdf)) and informally
since
[2014](https://web.archive.org/web/20160220215151/https://bitcointalk.org/index.php?topic=511881.0).
The value of having a configurable delay period with recovery capability
in light of an unexpected spend has been widely recognized.

The only way to implement vaults given the existing consensus rules,
aside from [emulating vaults with large multisig
configurations](https://github.com/revault), is to use presigned
transactions created with a one-time-use key. This approach was first
demonstrated
[in 2020](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2020-April/017755.html).

Unfortunately, this approach has a number of practical shortcomings:

  - generating and securely deleting ephemeral keys, which are used to
    emulate the vault covenant, is required,
  - amounts and withdrawal patterns must be precommitted to,
  - there is a necessity to precommit to an address that the funds must
    pass through on their way to the final withdrawal target, which is
    likely only known at unvault time,
  - the particular fee management technique or wallet must be decided
    upon vault creation,
  - coin loss follows if a vault address is reused,
  - the transaction data that represents the "bearer asset" of the vault
    must be stored for perpetuity, otherwise value is lost, and
  - the vault creation ceremony must be performed each time a new
    balance is to be deposited.

The deployment of a "precomputed" covenant mechanism like
[OP\_CHECKTEMPLATEVERIFY](https://github.com/bitcoin/bips/blob/master/bip-0119.mediawiki)
or
[SIGHASH\_ANYPREVOUT](https://github.com/bitcoin/bips/blob/master/bip-0118.mediawiki)
would both remove the necessity to use an ephemeral key, since the
covenant is enforced on-chain, and lessen the burden of sensitive data
storage, since the necessary transactions can be generated from a set of
compact parameters. This approach was demonstrated [in
2022](https://github.com/jamesob/simple-ctv-vault).

However, the limitations of precomputation still apply: amounts,
destinations, and fee management are all fixed. Funds must flow through
a fixed intermediary to their final destination. Batch operations, which
may be vital for successful recovery during fee spikes or short spend
delay, are not possible.

![bip-0345/withdrawal-comparison.drawio.png](bip-0345/withdrawal-comparison.drawio.png
"bip-0345/withdrawal-comparison.drawio.png")

Having a "general" covenant mechanism that can encode arbitrary
transactional state machines would allow us to solve these issues, but
at the cost of complex and large scripts that would probably be
duplicated many times over in the blockchain. The particular design and
deployment timeline of such a general framework is also uncertain. This
approach was demonstrated
[in 2016](https://blog.blockstream.com/en-covenants-in-elements-alpha/).

This proposal intends to address the problems outlined above by
providing a delay period/recovery path use with minimal transactional
and operational overhead using a specialized covenant.

The design goals of the proposal are:

  - **efficient reuse of an existing vault configuration.**\[1\] A
    single vault configuration, whether the same literal `scriptPubKey`
    or not, should be able to “receive” multiple deposits.

<!-- end list -->

  - **batched operations** for recovery and withdrawal to allow managing
    multiple vault coins efficiently.

<!-- end list -->

  - **unbounded partial withdrawals**, which allows users to withdraw
    partial vault balances without having to perform the setup ceremony
    for a new vault.

<!-- end list -->

  - **dynamic unvault targets**, which allow the proposed withdrawal
    target for a vault to be specified at withdrawal time rather than
    when the vault is first created. This would remove the need for a
    prespecified, intermediate wallet that only exists to route
    unvaulted funds to their desired destination.

<!-- end list -->

  - **dynamic fee management** that, like dynamic targets, defers the
    specification of fee rates and source to unvault time rather than
    vault creation time.

These goals are accompanied by basic safety considerations (e.g. not
being vulnerable to mempool pinning) and a desire for concision, both in
terms of the number of outputs created as well as script sizes.

This proposal is designed to be compatible with any future sighash modes
(e.g. `SIGHASH_GROUP`) or fee management strategies (e.g. [transaction
sponsors](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2020-September/018168.html))
that may be introduced. Use of these opcodes will benefit from, but do
not strictly rely on, [v3 transaction
relay](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2022-September/020937.html)
and [ephemeral
anchors](https://github.com/instagibbs/bips/blob/ephemeral_anchor/bip-ephemeralanchors.mediawiki).

## Design

In typical usage, a vault is created by encumbering coins under a
taptree
[(BIP-341)](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
containing at least two leaves: one with an `OP_VAULT`-containing script
that facilitates the expected withdrawal process, and another leaf with
`OP_VAULT_RECOVER` which ensures the coins can be recovered at any time
prior to withdrawal finalization.

The rules of `OP_VAULT` ensure the timelocked, interruptible withdrawal
by allowing a spending transaction to replace the `OP_VAULT` tapleaf
with a prespecified script template, allowing for some parameters to be
set at spend (trigger) time. All other leaves in the taptree must be
unchanged in the destination output, which preserves the recovery path
as well as any other spending conditions originally included in the
vault. This is similar to the `TAPLEAF_UPDATE_VERIFY` design that was
proposed
[in 2021](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2021-September/019419.html).

These tapleaf replacement rules, described more precisely below, ensure
a timelocked withdrawal, where the timelock is fixed by the original
`OP_VAULT` parameters, to a fixed set of outputs (via
`OP_CHECKTEMPLATEVERIFY`\[2\]) which is chosen when the withdrawal
process is triggered.

While `OP_CHECKTEMPLATEVERIFY` is used in this proposal as the preferred
method to bind the proposed withdrawal to a particular set of final
outputs, `OP_VAULT` is composable with other (and future) opcodes to
facilitate other kinds of withdrawal processes.

![bip-0345/opvault.drawio.png](bip-0345/opvault.drawio.png
"bip-0345/opvault.drawio.png")

### Transaction types

The vault has a number of stages, some of them optional:

  - **vault transaction**: encumbers some coins into a Taproot structure
    that includes at least one `OP_VAULT` leaf and one
    `OP_VAULT_RECOVER` leaf.

<!-- end list -->

  - **trigger transaction**: spends one or more `OP_VAULT`-tapleaf
    inputs into an output which is encumbered by a timelocked withdrawal
    to a fixed set of outputs, chosen at trigger time. This publicly
    broadcasts the intent to withdraw to some specific set of outputs.  
      
    The trigger transaction may have an additional output which
    allocates some of the vault balance into a partial "revault," which
    simply encumbers the revaulted portion of the value into the same
    `scriptPubKey` as the `OP_VAULT`-containing input(s) being spent.

<!-- end list -->

  - **withdrawal transaction**: spends the timelocked,
    destination-locked trigger inputs into a compatible set of final
    withdrawal outputs (per, e.g., a `CHECKTEMPLATEVERIFY` hash), after
    the trigger inputs have matured per the spend delay. Timelocked CTV
    transactions are the motivating usage of OP\_VAULT, but any script
    template can be specified during the creation of the vault.

<!-- end list -->

  - **recovery transaction**: spends one or more vault inputs via
    `OP_VAULT_RECOVER` tapleaf to the prespecified recovery path, which
    can be done at any point before the withdrawal transaction confirms.
    Each input can optionally require a witness satisfying a specified
    *recovery authorization* script, an optional script prefixing the
    `OP_VAULT_RECOVER` fragment. The use of recovery authorization has
    certain trade-offs discussed later.

### Fee management

A primary consideration of this proposal is how fee management is
handled. Providing dynamic fee management is critical to the operation
of a vault, since

  - precalculated fees are prone to making transactions unconfirmable in
    high fee environments, and
  - a fee wallet that is prespecified might be compromised or lost
    before use.

But dynamic fee management can introduce [pinning
vectors](https://bitcoinops.org/en/topics/transaction-pinning/). Care
has been taken to avoid unnecessarily introducing these vectors when
using the new destination-based spending policies that this proposal
introduces.

Originally, this proposal had a hard dependency on reformed transaction
nVersion=3 policies, including ephemeral anchors, but it has since been
revised to simply benefit from these changes in policy as well as other
potential fee management mechanisms.

## Specification

The tapscript opcodes `OP_SUCCESS187` (`0xbb`) and `OP_SUCCESS188`
(`0xbc`) are constrained with new rules to implement `OP_VAULT` and
`OP_VAULT_RECOVER`, respectively.

### `OP_VAULT` evaluation

When evaluating `OP_VAULT` (`OP_SUCCESS187`, `0xbb`), the expected
format of the stack, shown top to bottom, is:

    <leaf-update-script-body>
    <push-count>
    [ <push-count> leaf-update script data items ... ]
    <trigger-vout-idx> 
    <revault-vout-idx>
    <revault-amount>

where

  - <leaf-update-script-body> is a minimally-encoded data push of a
    serialized script. \[3\]
      - Otherwise, script execution MUST fail and terminate immediately.

<!-- end list -->

  - <push-count> is an up to 4-byte minimally encoded `CScriptNum`
    indicating how many leaf-update script items should be popped off
    the stack. \[4\]
      - If this value does not decode to a valid CScriptNum, script
        execution MUST fail and terminate immediately.
      - If this value is less than 0, script execution MUST fail and
        terminate immediately.
      - If there are fewer than 3 items following the <push-count> items
        on the stack, script execution MUST fail and terminate
        immediately. In other words, after popping
        <leaf-update-script-body>, there must be at least ` 3 +
         `<push-count> items remaining on the stack.

<!-- end list -->

  - The following <push-count> stack items are popped off the stack and
    prefixed as minimally-encoded push-data arguments to the
    <leaf-update-script-body> to construct the expected tapleaf
    replacement script.

<!-- end list -->

  - <trigger-vout-idx> is an up to 4-byte minimally encoded `CScriptNum`
    indicating the index of the output which, in conjunction with an
    optional revault output, carries forward the value of this input,
    and has an identical taptree aside from the currently executing
    leaf.
      - If this value does not decode to a valid CScriptNum, script
        execution MUST fail and terminate immediately.
      - If this value is less than 0 or is greater than or equal to the
        number of outputs, script execution MUST fail and terminate
        immediately.

<!-- end list -->

  - <revault-vout-idx> is an up to 4-byte minimally encoded `CScriptNum`
    optionally indicating the index of an output which, in conjunction
    with the trigger output, carries forward the value of this input,
    and has an identical scriptPubKey to the current input.
      - If this value does not decode to a valid CScriptNum, script
        execution MUST fail and terminate immediately.
      - If this value is greater than or equal to the number of outputs,
        script execution MUST fail and terminate immediately.
      - If this value is negative and not equal to -1, script execution
        MUST fail and terminate immediately.\[5\]

<!-- end list -->

  - <revault-amount> is an up to 7-byte minimally encoded CScriptNum
    indicating the number of satoshis being revaulted.
      - If this value does not decode to a valid CScriptNum, script
        execution MUST fail and terminate immediately.
      - If this value is not greater than or equal to 0, script
        execution MUST fail and terminate immediately.
      - If this value is non-zero but <revault-vout-idx> is negative,
        script execution MUST fail and terminate immediately.
      - If this value is zero but <revault-vout-idx> is not -1, script
        execution MUST fail and terminate immediately.

After the stack is parsed, the following validation checks are
performed:

  - Decrement the per-script sigops budget (see
    [BIP-0342](https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki#user-content-Resource_limits))
    by 60\[6\]; if the budget is brought below zero, script execution
    MUST fail and terminate immediately.
  - Let the output designated by <trigger-vout-idx> be called
    *triggerOut*.
  - If the scriptPubKey of *triggerOut* is not a version 1 witness
    program, script execution MUST fail and terminate immediately.
  - Let the script constructed by taking the <leaf-update-script-body>
    and prefixing it with minimally-encoded data pushes of the
    <push-count> leaf-update script data items be called the
    *leaf-update-script*.
  - If the scriptPubKey of *triggerOut* does not match that of a taptree
    that is identical to that of the currently evaluated input, but with
    the leaf script substituted for *leaf-update-script*, script
    execution MUST fail and terminate immediately.
      - Note: the parity bit of the resulting taproot output is allowed
        to vary, so both values for the new output must be checked.
  - Let the output designated by <revault-vout-idx> (if the index value
    is non-negative) be called *revaultOut*.
  - If the scriptPubKey of *revaultOut* is not equal to the scriptPubKey
    of the input being spent, script execution MUST fail and terminate
    immediately.
  - Implementation recommendation: if the sum of the amounts of
    *triggerOut* and *revaultOut* (if any) are not greater than or equal
    to the value of this input, script execution SHOULD fail and
    terminate immediately. This ensures that (at a minimum) the vaulted
    value for this input is carried through.
      - Amount checks are ultimately done with deferred checks, but this
        check can help short-circuit obviously invalid spends.
  - Queue a deferred check\[7\] that ensures the satoshis for this
    input's `nValue` minus <revault-amount> are included within the
    output `nValue` found at <trigger-vout-idx>.
  - Queue a deferred check that ensures <revault-amount> satoshis, if
    non-zero, are included within the output's `nValue` found at
    <revault-vout-idx>.
      - These deferred checks could be characterized in terms of the
        pseudocode below (in *Deferred checks*) as  
        ` TriggerCheck(input_amount,  `<revault-amount>` ,
         `<trigger-vout-idx>` ,  `<revault-vout-idx>`)`.

If none of the conditions fail, a single true value (`0x01`) is left on
the stack.

### `OP_VAULT_RECOVER` evaluation

When evaluating `OP_VAULT_RECOVER` (`OP_SUCCESS188`, `0xbb`), the
expected format of the stack, shown top to bottom, is:

    <recovery-sPK-hash>
    <recovery-vout-idx>

where

  - <recovery-sPK-hash> is a 32-byte data push.
      - If this is not 32 bytes in length, script execution MUST fail
        and terminate immediately.
  - <recovery-vout-idx> is an up to 4-byte minimally encoded
    `CScriptNum` indicating the index of the recovery output.
      - If this value does not decode to a valid CScriptNum, script
        execution MUST fail and terminate immediately.
      - If this value is less than 0 or is greater than or equal to the
        number of outputs, script execution MUST fail and terminate
        immediately.

After the stack is parsed, the following validation checks are
performed:

  - Let the output at index <recovery-vout-idx> be called *recoveryOut*.
  - If the scriptPubKey of *recoveryOut* does not have a tagged hash
    equal to <recovery-sPK-hash> (`tagged_hash("VaultRecoverySPK",
    recoveryOut.scriptPubKey) == recovery-sPK-hash`, where
    `tagged_hash()` is from the [BIP-0340 reference
    code](https://github.com/bitcoin/bips/blob/master/bip-0340/reference.py)),
    script execution MUST fail and terminate immediately.
      - Implementation recommendation: if *recoveryOut* does not have an
        `nValue` greater than or equal to this input's amount, the
        script SHOULD fail and terminate immediately.
  - Queue a deferred check that ensures the `nValue` of *recoveryOut*
    contains the entire `nValue` of this input.\[8\]
      - This deferred check could be characterized in terms of the
        pseudocode below as `RecoveryCheck(`<recovery-vout-idx>`,
        input_amount)`.

If none of the conditions fail, a single true value (`0x01`) is left on
the stack.

### Deferred check evaluation

Once all inputs for a transaction are validated per the rules above, any
deferred checks queued MUST be evaluated.

The Python pseudocode for this is as follows:

``` python
class TriggerCheck:
    """Queued by evaluation of OP_VAULT (withdrawal trigger)."""
    input_amount: int
    revault_amount: int
    trigger_vout_idx: int
    revault_vout_idx: int


class RecoveryCheck:
    """Queued by evaluation of OP_VAULT_RECOVER."""
    input_amount: int
    vout_idx: int


def validate_deferred_checks(checks: [DeferredCheck], tx: Transaction) -> bool:
    """
    Ensure that all value from vault inputs being triggered or recovered is preserved
    in suitable output nValues.
    """
    # Map to hold expected output values.
    out_map: Dict[int, int] = defaultdict(lambda: 0)

    for c in checks:
        if isinstance(c, TriggerCheck):
            out_map[c.trigger_vout_idx] += (c.input_amount - c.revault_amount)

            if c.revault_amount > 0:
                out_map[c.revault_vout_idx] += c.revault_amount

        elif isinstance(c, RecoveryCheck):
            out_map[c.vout_idx] += c.input_amount

    for (vout_idx, amount_sats) in out_map.items():
        # Trigger/recovery value can be greater than the constituent vault input
        # amounts.
        if tx.vout[vout_idx].nValue < amount_sats:
            return False

    return True
```

If the above procedure, or an equivalent, returns false, script
execution MUST fail and terminate immediately.

This ensures that all compatible vault inputs can be batched into shared
corresponding trigger or recovery outputs while preserving their entire
input value.

## Policy changes

In order to prevent possible pinning attacks, recovery transactions must
be replaceable.

  - When validating an `OP_VAULT_RECOVER` input being spent, the script
    MUST fail (by policy, not consensus) and terminate immediately if
    both\[9\]
    1.  the input is not marked as opt-in replaceable by having an
        nSequence number less than `0xffffffff - 1`, per
        [BIP-0125](https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki),
        and
    2.  the version of the recovery transaction has an nVersion other
        than 3.

If the script containing `OP_VAULT_RECOVER` is 34 bytes or less\[10\],
let it be called "unauthorized," because there is no script guarding the
recovery process. In order to prevent pinning attacks in the case of
unauthorized recovery - since the spend of the input (and the structure
of the transaction) is not authorized by a signed signature message -
the output structure of unauthorized recovery transaction is limited.

  - If the recovery is unauthorized, the recovery transaction MUST (by
    policy) abide by the following constraints:
      - If the spending transaction has more than two outputs, the
        script MUST fail and terminate immediately.
      - If the spending transaction has two outputs, and the output
        which is not *recoveryOut* is not an [ephemeral
        anchor](https://github.com/instagibbs/bips/blob/ephemeral_anchor/bip-ephemeralanchors.mediawiki),
        the script MUST fail and terminate immediately.\[11\]

## Implementation

A sample implementation is available on bitcoin-inquisition
[here](https://github.com/jamesob/bitcoin/tree/2023-01-opvault-inq),
with an associated [pull
request](https://github.com/bitcoin-inquisition/bitcoin/pull/21).

## Applications

The specification above, perhaps surprisingly, does not specifically
cover how a relative timelocked withdrawal process with a fixed target
is implemented. The tapleaf update semantics specified in `OP_VAULT` as
well as the output-based authorization enabled by `OP_VAULT_RECOVER` can
be used to implement a vault, but they are incomplete without two other
pieces:

  - a way to enforce relative timelocks, like `OP_CHECKSEQUENCEVERIFY`,
    and
  - a way to enforce that proposed withdrawals are ultimately being
    spent to a precise set of outputs, like `OP_CHECKTEMPLATEVERIFY`.

These two pieces are combined with the tapleaf update capabilities of
`OP_VAULT` to create a vault, described below.

### Creating a vault

In order to vault coins, they can be spent into a witness v1
`scriptPubKey` that contains a taptree of the form

    tr(<internal-pubkey>,
      leaves = {
        recover: 
          <recovery-sPK-hash> OP_VAULT_RECOVER,
    
        trigger: 
          <trigger-auth-pubkey> OP_CHECKSIGVERIFY                     (i)
          <spend-delay> 2 $leaf-update-script-body OP_VAULT,          (ii)
    
        ... [ possibly other leaves ]
      }
    )

where

  - `$leaf-update-script-body` is, for example, `OP_CHECKSEQUENCEVERIFY
    OP_DROP OP_CHECKTEMPLATEVERIFY`.
      - This is one example of a trigger script, but *any* script
        fragment can be used, allowing the creation of different types
        of vaults. For example, you could use `OP_CHECKSEQUENCEVERIFY
        OP_DROP OP_CHECKSIG` to do a time-delayed transfer of the coins
        to another key. This also future-proofs `OP_VAULT` for future
        scripting capabilities.
  - The script fragment in `(i)` is called the "trigger authorization,"
    because it gates triggering the withdrawal. This can be done in
    whatever manner the wallet designer would like.
  - The script fragment in `(ii)` is the incomplete `OP_VAULT`
    invocation - it will be completed once the rest of the parameters
    (the CTV target hash, trigger vout index, and revault vout index)
    are provided by the trigger transaction witness.

Typically, the internal key for the vault taproot output will be
specified so that it is controlled by the same descriptor as the
recovery path, which facilitates another (though probably unused) means
of recovering the vault output to the recovery path. This has the
potential advantage of recovering the coin without ever revealing it was
a vault.

Otherwise, the internal key can be chosen to be an unspendable NUMS
point to force execution of the taptree contents.

### Triggering a withdrawal

To make use of the vault, and spend it towards some output, we construct
a spend of the above `tr()` output that simply replaces the "trigger"
leaf with the full leaf-update script (in this case, a timelocked CTV
script):

    Witness stack:
    
    - <revault-amount>
    - <revault-vout-idx> (-1 if none)
    - <trigger-vout-idx>
    - <target-CTV-hash>
    - <trigger-auth-pubkey-signature>
    - [ "trigger" leaf script contents ]
    - [ taproot control block prompting a script-path spend to "trigger" leaf ]
    
    Output scripts:
    
    [
      tr(<internal-pubkey>,
        leaves = {
          recover: 
            <recovery-sPK-hash> OP_VAULT_RECOVER,               <--  unchanged
    
          trigger:
            <target-CTV-hash> <spend-delay> 
            OP_CHECKSEQUENCEVERIFY OP_DROP OP_CHECKTEMPLATEVERIFY  <--  changed per the 
                                                                        leaf-update
                                                                        rules of OP_VAULT
           ... [ possibly other leaves ]
         }
       ),                                                               
    
       [ optional revault output with the
         same sPK as the original vault output ],
    ]

`OP_VAULT` has allowed the taptree to be transformed so that the trigger
leaf becomes a timelocked CTV script, which is what actually facilitates
the announced withdrawal. The withdrawal is interruptible by the
recovery path because the "recover" leaf is preserved exactly from the
original taptree.

Note that the CTV hash is specified at spend time using the witness
stack, and "locked in" via the `OP_VAULT` spend rules which assert its
existence in the output.

The vault funds can be recovered at any time prior to the spend of the
timelocked CTV script by way of a script-path spend using the "recover"
leaf.

### Recovery authorization

When configuring a vault, the user must decide if they want to have the
recovery process gated by a script fragment prefixing the
`OP_VAULT_RECOVER` instruction in the "recover" leaf. Its use entails
trade-offs.

#### Unauthorized recovery

Unauthorized recovery simplifies vault use in that recovery never
requires additional information aside from the location of the vault
outpoints and the recovery path - the "authorization" is simply the
reveal of the recovery path, i.e. the preimage of <recovery-sPK-hash>.

But because this reveal is the only authorization necessary to spend the
vault coins to recovery, the user must expect to recover all such vaults
at once, since an observer can replay this recovery (provided they know
the outpoints).

Additionally, unauthorized recovery across multiple distinct recovery
paths cannot be done in the same transaction, and fee control is more
constrained: because the output structure is limited for unauthorized
recovery, fee management relies either on inputs which are completely
spent to fees or the use of the optional ephemeral anchor and package
relay.

These limitations are to avoid pinning attacks.

#### Authorized recovery

With authorized recovery, the user must keep track of an additional
piece of information: how to solve the recovery authorization script
fragment when recovery is required.

If this key is lost, the user will be unable to initiate the recovery
process for their coins. If an attacker obtains the recovery key, they
may grief the user during the recovery process by constructing a low fee
rate recovery transaction and broadcasting it (though they will not be
able to pin because of the replaceability requirement on recovery
transactions).

However, authorized recovery configurations have significant benefits.
Batched recoveries are possible for vaults with otherwise incompatible
recovery parameters. Fee management is much more flexible, since
authorized recovery transactions are "free form" and unrelated inputs
and outputs can be added, potentially to handle fees.

#### Recommendation: use a simple, offline recovery authorization key seed

The benefits of batching and fee management that authorized recovery
provides are significant. If the recovery authorization key falls into
the hands of an attacker, the outcome is not catastrophic, whereas if
the user loses their recovery authorization key as well as their trigger
key, the result is likely coin loss. Consequently, the author's
recommendation is to use a simple seed for the recovery authorization
key that can be written down offline and replicated.

Note that the recovery authorization key **is not** the recovery path
key, and this is **much different** than any recommendation on how to
generate the recovery path key itself.

### Address reuse and recovery

When creating a vault, four factors affect the resulting P2TR address:

1.  The internal pubkey (likely belonging to the recovery wallet)
2.  The recovery leaf
3.  The trigger leaf
4.  Any other leaves that exist in the taptree

The end user has the option of varying certain contents along
descriptors in order to avoid reusing vault addresses without affecting
key management, e.g. the trigger authorization pubkeys.

Note that when using unauthorized recovery, the reveal of the recovery
scriptPubKey will allow any observer to initiate the recovery process
for any vault with matching recovery params, provided they are able to
locate the vault outpoints. As a result, it is recommended to expect
that **all outputs sharing an identical unauthorized <recovery-sPK-hash>
should be recovered together**.

This situation can be avoided with a comparable key management model by
varying the generation of each vault's recovery scriptPubKey along a
single descriptor, but note that this will prevent recovering multiple
separate vaults into a single recovery output.

Varying the internal pubkey will prevent batching the trigger of
multiple vault inputs into a single trigger output; consequently it is
recommended that users instead vary some component of the trigger leaf
script if address reuse is undesirable. Users could vary the trigger
pubkey along a descriptor, keeping the recovery path and internal-pubkey
the same, which both avoids reusing addresses and allows batched trigger
and recovery operations.

#### Recommendation: generate new recovery addresses for new trigger keys

If using unauthorized recovery, it is recommended that you do not share
recovery scriptPubKeys across separate trigger keys. If one trigger key
is compromised, that will necessitate the (unauthorized) recovery of all
vaults with that trigger key, which will reveal the recovery path
preimage. This means that an observer might be able to initiate recovery
for vaults controlled by an uncompromised trigger key.

#### Fee management

Fees can be managed in a variety of ways, but it's worth noting that
both trigger and recovery transactions must preserve the total value of
vault inputs, so vaulted values cannot be repurposed to pay for fees.
This does not apply to the withdrawal transaction, which can allocate
value arbitrarily.

In the case of vaults that use recovery authorization, all transactions
can "bring their own fees" in the form of unrelated inputs and outputs.
These transactions are also free to specify ephemeral anchors, once the
related relay policies are deployed. This means that vaults using
recovery authorization have no dependence on the deploy of v3 relay
policy.

For vaults using unauthorized recovery, the recovery transaction relies
on the use of either fully-spent fee inputs or an ephemeral anchor
output. This means that vaults which do not use recovery authorization
are essentially dependent on v3 transaction relay policy being deployed.

### Batching

#### During trigger

`OP_VAULT` outputs with the same taptree, aside from slightly different
trigger leaves, can be batched together in the same withdrawal process.
Two "trigger" leaves are compatible if they have the same `OP_VAULT`
arguments.

Note that this allows the trigger authorization -- the script prefixing
the `OP_VAULT` invocation -- to differ while still allowing batching.

Trigger transactions can act on multiple incompatible `OP_VAULT` input
sets, provided each set has a suitable associated *triggerOut* output.

Since `SIGHASH_DEFAULT` can be used to sign the trigger authorization,
unrelated inputs and outputs can be included, possibly to facilitate fee
management or the batch withdrawal of incompatible vaults.

#### During withdrawal

During final withdrawal, multiple trigger outputs can be used towards
the same withdrawal transaction provided that they share identical
<target-CTV-hash> parameters. This facilitates batched withdrawals.

#### During recovery

`OP_VAULT_RECOVER` outputs with the same <recovery-sPK-hash> can be
recovered into the same output.

Recovery-incompatible vaults which have authorized recovery can be
recovered in the same transaction, so long as each set (grouped by
<recovery-sPK-hash>) has an associated *recoveryOut*. This allows
unrelated recoveries to share common fee management.

### Watchtowers

The value of vaults is contingent upon having monitoring in place that
will alert the owner when unexpected spends are taking place. This can
be done in a variety of ways, with varying degrees of automation and
trust in the watchtower.

In the maximum-trust case, the watchtower can be fully aware of all
vaulted coins and has the means to initiate the recovery process if
spends are not pre-reported to the watchtower.

In the minimum-trust case, the user can supply a probabilistic filter of
which coins they wish to monitor; the watchtower would then alert the
user if any coins matching the filter move, and the user would be
responsible for ignoring false positives and handling recovery
initiation.

### Output descriptors

Output descriptors for vault-related outputs will be covered in a
subsequent BIP.

## Deployment

Activation mechanism is to be determined.

This BIP should be deployed concurrently with BIP-0119 to enable full
use of vaults.

## Backwards compatibility

`OP_VAULT` and `OP_VAULT_RECOVER` replace, respectively, the witness
v1-only opcodes OP\_SUCCESS187 and OP\_SUCCESS188 with stricter
verification semantics. Consequently, scripts using those opcodes which
previously were valid will cease to be valid with this change.

Stricter verification semantics for an OP\_SUCCESSx opcode are a soft
fork, so existing software will be fully functional without upgrade
except for mining and block validation.

Backwards compatibility considerations are very comparable to previous
deployments for OP\_CHECKSEQUENCEVERIFY and OP\_CHECKLOCKTIMEVERIFY (see
[BIP-0065](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
and
[BIP-0112](https://github.com/bitcoin/bips/blob/master/bip-0112.mediawiki)).

## Rationale

<references />

## References

  - [\[bitcoin-dev](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2016-February/012470.html)
    Bitcoin Vaults (2016)\]
  - [\[bitcoin-dev](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2018-February/015793.html)
    Simple lock/unlock mechanism (2018)\]
  - [\[bitcoin-dev](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2020-April/017755.html)
    On-chain vaults prototype (2020)\]
  - [\[bitcoin-dev](https://lists.linuxfoundation.org/pipermail/bitcoin-dev/2021-September/019419.html)
    TAPLEAF\_UPDATE\_VERIFY covenant opcode (2021)\]
  - [Custody Protocols Using Bitcoin Vaults
    (2020)](https://arxiv.org/abs/2005.11776)
  - [Vaults and Covenants (2023)](https://jameso.be/vaults.pdf)

## Acknowledgements

The author would like to thank

  - AJ Towns and Greg Sanders for discussion, numerous suggestions that
    improved the proposal, and advice.
  - Jeremy Rubin for inspiration, advice, and mentorship.
  - BL for discussion and insight.
  - John Moffett for early feedback and a test case demonstrating a
    recursive script evaluation attack.
  - Johan Halseth for providing conceptual review and pointing out a
    pinning attack.
  - Pieter Wuille for implementation advice.

<!-- end list -->

1.  **Why does this support address reuse?** The proposal doesn't rely
    on or encourage address reuse, but certain uses are unsafe if
    address reuse cannot be handled - for example, if a custodian gives
    its users a vault address to deposit to, it cannot enforce that
    those users make a single deposit for each address.
2.  **Why is `OP_CHECKTEMPLATEVERIFY` (BIP-119) relied upon for this
    proposal?** During the withdrawal process, the proposed final
    destination for value being withdrawn must be committed to. `OP_CTV`
    is the simplest, safest way to commit the spend of some coins to a
    particular set of outputs. An earlier version of this proposal
    attempted to use a simpler, but similar method, of locking the spend
    of coins to a set of outputs, but this method introduced txid
    malleability.  
    Note that if some other method of locking spends to a particular set
    of outputs should be deployed, that method can be used in the
    `OP_VAULT` <leaf-update-script-body> with no changes.
3.  In conjunction with the leaf-update data items, it dictates the
    tapleaf script in the output taptree that will replace the one
    currently executing.
4.  **Why only prefix with data pushes?** Prefixing the
    `leaf-update-script-body` with opcodes opens up the door to prefix
    OP\_SUCCESSX opcodes, to name a single issue only, side-stepping the
    validation that was meant to be run by the committed script.
5.  **Why is -1 the only allowable negative value for
    revault-vout-idx?** A negative revault index indicates that no
    revault output exists; if this value were allowed to be any negative
    number, the witness could be malleated (and bloated) while a
    transaction is waiting for confirmation.
6.  **Why is the sigops cost for OP\_VAULT set to 60?** To determine the
    validity of a trigger output, OP\_VAULT must perform an EC
    multiplication and hashing proportional to the length of the control
    block in order to generate the output's expected TapTweak. This has
    been measured to have a cost in the worst case (max length control
    block) of roughly twice a Schnorr verification. Because the hashing
    cost could be mitigated by caching midstate, the cost is 60 and not
    100.
7.  **What is a deferred check and why does this proposal require them
    for correct script evaluation?** A deferred check is a validation
    check that is executed only after all input scripts have been
    validated, and is based on aggregate information collected during
    each input's EvalScript run.  
      
    Currently, the validity of each input is (usually) checked
    concurrently across all inputs in a transaction. Because this
    proposal allows batching the spend of multiple vault inputs into a
    single recovery or withdrawal output, we need a mechanism to ensure
    that all expected values per output can be summed and then checked.
    This necessitates the introduction of an "aggregating" set of checks
    which can only be executed after each input's script is evaluated.
    Note that similar functionality would be required for batch input
    validation or cross-input signature aggregation.
8.  **How do recovery transactions pay for fees?** If the recovery is
    unauthorized, fees are attached either via CPFP with an ephemeral
    anchor or as inputs which are solely spent to fees (i.e. no change
    output). If the recovery is authorized, fees can be attached in any
    manner, e.g. unrelated inputs and outputs or CPFP via anchor.
9.  **Why are recovery transactions required to be replaceable?** In the
    case of unauthorized recoveries, an attacker may attempt to pin
    recovery transactions by broadcasting a "rebundled" version with a
    low fee rate. Vault owners must be able to overcome this with
    replacement. In the case of authorized recovery, if an attacker
    steals the recovery authorization key, the attacker may try to pin
    the recovery transaction during theft. Requiring replaceability
    ensures that the owner can always raise the fee rate of the recovery
    transaction, even if they are RBF rule \#3 griefed in the process.
10. 34 bytes is the length of a recovery script that consists solely of
    <recovery-sPK-hash>`  OP_VAULT_RECOVER `.
11. **Why can unauthorized recoveries only process a single recovery
    path?** Because there is no signature required for unauthorized
    recoveries, if additional outputs were allowed, someone observing a
    recovery in the mempool would be able to rebundle and broadcast the
    recovery with a lower fee rate.
