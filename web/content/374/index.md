
+++
title = "Discrete Log Equality Proofs"
date = 2024-12-26
weight = 374

[taxonomies]
authors = ["Andrew Toth", "Ruben Somsen", "Sebastian Falbesoner"]
status = ["Draft"]

[extra]
bip = 374
status = ["Draft"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0374.mediawiki"
note = "THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING"
+++

```
  BIP: 374
  Layer: Applications
  Title: Discrete Log Equality Proofs
  Author: Andrew Toth <andrewstoth@gmail.com>
          Ruben Somsen <rsomsen@gmail.com>
          Sebastian Falbesoner <sebastian.falbesoner@gmail.com>
  Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0374
  Status: Draft
  Type: Standards Track
  License: BSD-2-Clause
  Created: 2024-12-26
  Post-History: https://gist.github.com/andrewtoth/df97c3260cc8d12f09d3855ee61322ea
                https://groups.google.com/g/bitcoindev/c/MezoKV5md7s
```

<h2> Introduction </h2>


<h3> Abstract </h3>


This document proposes a standard for 64-byte zero-knowledge _discrete logarithm equality proofs_ (DLEQ proofs) over an elliptic curve. For given elliptic curve points _A_, _B_, _C_, _G_, and a scalar _a_ known only to the prover where _A = a⋅G_ and _C = a⋅B_, the prover proves knowledge of _a_ without revealing anything about _a_. This can, for instance, be useful in ECDH: if _A_ and _B_ are ECDH public keys, and _C_ is their ECDH shared secret computed as _C = a⋅B_, the proof establishes that the same secret key _a_ is used for generating both _A_ and _C_ without revealing _a_.

<h3> Copyright </h3>


This document is licensed under the 2-clause BSD license.

<h3> Motivation </h3>


<a href="/352" target="_blank">BIP352</a> requires senders to compute output scripts using ECDH shared secrets from the same secret keys used to sign the inputs. Generating an incorrect signature will produce an invalid transaction that will be rejected by consensus. An incorrectly generated output script can still be consensus-valid, meaning funds may be lost if it gets broadcast.
By producing a DLEQ proof for the generated ECDH shared secrets, the signing entity can prove to other entities that the output scripts have been generated correctly without revealing the private keys.

<h2> Specification </h2>


All conventions and notations are used as defined in <a href="/327" target="_blank">BIP327</a>.

<h3> Description </h3>


The basic proof generation uses a random scalar _k_, the secret _a_, and the point being proven _C = a⋅B_.

*  Let _R<sub>1</sub> = k⋅G_.
*  Let _R<sub>2</sub> = k⋅B_.
*  Let _e = hash(R<sub>1</sub> || R<sub>2</sub>)_.
*  Let _s = (k + e⋅a)_.


Providing only _C_, _e_ and _s_ as a proof does not reveal _a_ or _k_.

Verifying the proof involves recreating _R<sub>1</sub>_ and _R<sub>2</sub>_ with only _e_ and _s_ as follows:

*  Let _R<sub>1</sub> = s⋅G - e⋅A_.
*  Let _R<sub>2</sub> = s⋅B - e⋅C_.


This can be verified by substituting _s = (k + e⋅a)_:

*  _s⋅G - e⋅A = (k + e⋅a)⋅G - e⋅A = k⋅G + e⋅(a⋅G) - e⋅A = k⋅G + e⋅A - e⋅A = k⋅G_.
*  _s⋅B - e⋅C = (k + e⋅a)⋅B - e⋅C = k⋅B + e⋅(a⋅B) - e⋅C = k⋅B + e⋅C - e⋅C = k⋅B_.


Thus verifying _e = hash(R<sub>1</sub> || R<sub>2</sub>)_ proves the discrete logarithm equivalency of _A_ and _C_.

<h3> DLEQ Proof Generation </h3>


The following generates a proof that the result of _a⋅B_ and the result of _a⋅G_ are both generated from the same scalar _a_ without having to reveal _a_.

Input:
*  The secret key _a_: a 256-bit unsigned integer
*  The public key _B_: a point on the curve
*  Auxiliary random data _r_: a 32-byte array<ref name="why_include_auxiliary_random_data"> ** Why include auxiliary random data?** The auxiliary random data should be set to fresh randomness for each proof. The same rationale and recommendations from <a href="/340" target="_blank">default-signing BIP340</a> should be applied.</ref> 
*  The generator point _G_: a point on the curve<ref name="why_include_G"> ** Why include the generator point G as an input?** While all other BIPs have used the generator point from secp256k1, passing it as an input here lets this algorithm be used for other curves.</ref>
*  An optional message _m_: a 32-byte array<ref name="why_include_a_message"> ** Why include a message as an input?** This could be useful for protocols that want to authorize on a compound statement, not just knowledge of a scalar. This allows the protocol to combine knowledge of the scalar and the statement.</ref>


The algorithm _GenerateProof(a, B, r, G, m)_ is defined as:
*  Fail if _a = 0_ or _a &ge; n_.
*  Fail if _is_infinite(B)_.
*  Let _A = a⋅G_.
*  Let _C = a⋅B_.
*  Let _t_ be the byte-wise xor of _bytes(32, a)_ and _hash<sub>BIP0374/aux</sub>(r)_.
*  Let _rand = hash<sub>BIP0374/nonce</sub>(t || cbytes(A) || cbytes(C))_.
*  Let _k = int(rand) mod n_.
*  Fail if _k = 0_.
*  Let _R<sub>1</sub> = k⋅G_.
*  Let _R<sub>2</sub> = k⋅B_.
*  Let _m' = m if m is provided, otherwise an empty byte array_.
*  Let _e = int(hash<sub>BIP0374/challenge</sub>(cbytes(A) || cbytes(B) || cbytes(C) || cbytes(G) || cbytes(R<sub>1</sub>) || cbytes(R<sub>2</sub>) || m'))_.
*  Let _s = (k + e⋅a) mod n_.
*  Let _proof = bytes(32, e) || bytes(32, s)_.
*  If _VerifyProof(A, B, C, proof)_ (see below) returns failure, abort.
*  Return the proof _proof_.


<h3> DLEQ Proof Verification </h3>


The following verifies the proof generated in the previous section. If the following algorithm succeeds, the points _A_ and _C_ were both generated from the same scalar. The former from multiplying by _G_, and the latter from multiplying by _B_.

Input:
*  The public key of the secret key used in the proof generation _A_: a point on the curve
*  The public key used in the proof generation _B_: a point on the curve
*  The result of multiplying the secret and public keys used in the proof generation _C_: a point on the curve
*  A proof _proof_: a 64-byte array
*  The generator point used in the proof generation _G_: a point on the curve<ref name="why_include_G"> ** Why include the generator point G as an input?** While all other BIPs have used the generator point from Secp256k1, passing it as an input here lets this algorithm be used for other curves.</ref>
*  An optional message _m_: a 32-byte array<ref name="why_include_a_message"> ** Why include a message as an input?** This could be useful for protocols that want to authorize on a compound statement, not just knowledge of a scalar. This allows the protocol to combine knowledge of the scalar and the statement.</ref>


The algorithm _VerifyProof(A, B, C, proof, G, m)_ is defined as:
*  Fail if any of _is_infinite(A)_, _is_infinite(B)_, _is_infinite(C)_, _is_infinite(G)_
*  Let _e = int(proof[0:32])_.
*  Let _s = int(proof[32:64])_; fail if _s &ge; n_.
*  Let _R<sub>1</sub> = s⋅G - e⋅A_.
*  Fail if _is_infinite(R<sub>1</sub>)_.
*  Let _R<sub>2</sub> = s⋅B - e⋅C_.
*  Fail if _is_infinite(R<sub>2</sub>)_.
*  Let _m' = m if m is provided, otherwise an empty byte array_.
*  Fail if _e ≠ int(hash<sub>BIP0374/challenge</sub>(cbytes(A) || cbytes(B) || cbytes(C) || cbytes(G) || cbytes(R<sub>1</sub>) || cbytes(R<sub>2</sub>) || m'))_.
*  Return success iff no failure occurred before reaching this point.


<h2>Backwards Compatibility</h2>


This proposal is compatible with all older clients.

<h2> Test Vectors and Reference Code </h2>


A reference python implementation is included <a href="https://github.com/bitcoin/bips/blob/master/bip-0374/reference.py" target="_blank">here</a>.
Test vectors can be generated by running `./bip-0374/gen_test_vectors.py` which will produce a CSV file of random test vectors for both generating and verifying proofs. These can be run against the reference implementation with `./bip-0374/run_test_vectors.py`.

<h2> Footnotes </h2>


<references />

<h2> Acknowledgements </h2>


Thanks to josibake, Tim Ruffing, benma, stratospher, waxwing, Yuval Kogman and all others who
participated in discussions on this topic.