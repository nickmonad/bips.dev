+++
title = "Payment Protocol"
date = 2013-07-29
weight = 70
in_search_index = true

[taxonomies]
authors = ["Gavin Andresen", "Mike Hearn"]
status = ["Final"]

[extra]
bip = 70
status = ["Final"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0070.mediawiki"
+++

      BIP: 70
      Layer: Applications
      Title: Payment Protocol
      Author: Gavin Andresen <gavinandresen@gmail.com>
              Mike Hearn <mhearn@bitcoinfoundation.org>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0070
      Status: Final
      Type: Standards Track
      Created: 2013-07-29

## Abstract

This BIP describes a protocol for communication between a merchant and
their customer, enabling both a better customer experience and better
security against man-in-the-middle attacks on the payment process.

## Motivation

The current, minimal Bitcoin payment protocol operates as follows:

1.  Customer adds items to an online shopping basket, and decides to pay
    using Bitcoin.
2.  Merchant generates a unique payment address, associates it with the
    customer's order, and asks the customer to pay.
3.  Customer copies the Bitcoin address from the merchant's web page and
    pastes it into whatever wallet they are using OR follows a bitcoin:
    link and their wallet is launched with the amount to be paid.
4.  Customer authorizes payment to the merchant's address and broadcasts
    the transaction through the Bitcoin p2p network.
5.  Merchant's server detects payment and after sufficient transaction
    confirmations considers the transaction final.

This BIP extends the above protocol to support several new features:

1.  Human-readable, secure payment destinations-- customers will be
    asked to authorize payment to "example.com" instead of an
    inscrutable, 34-character bitcoin address.
2.  Secure proof of payment, which the customer can use in case of a
    dispute with the merchant.
3.  Resistance from man-in-the-middle attacks that replace a merchant's
    bitcoin address with an attacker's address before a transaction is
    authorized with a hardware wallet.
4.  Payment received messages, so the customer knows immediately that
    the merchant has received, and has processed (or is processing)
    their payment.
5.  Refund addresses, automatically given to the merchant by the
    customer's wallet software, so merchants do not have to contact
    customers before refunding overpayments or orders that cannot be
    fulfilled for some reason.

## Protocol

This BIP describes payment protocol messages encoded using Google's
Protocol Buffers, authenticated using X.509 certificates, and
communicated over http/https. Future BIPs might extend this payment
protocol to other encodings, PKI systems, or transport protocols.

The payment protocol consists of three messages; PaymentRequest,
Payment, and PaymentACK, and begins with the customer somehow indicating
that they are ready to pay and the merchant's server responding with a
PaymentRequest message:

<img src=bip-0070/Protocol_Sequence.png></img>

## Messages

The Protocol Buffers messages are defined in
[paymentrequest.proto](bip-0070/paymentrequest.proto "wikilink").

### Output

Outputs are used in PaymentRequest messages to specify where a payment
(or part of a payment) should be sent. They are also used in Payment
messages to specify where a refund should be sent.

        message Output {
        optional uint64 amount = 1 [default = 0];
            optional bytes script = 2;
        }

|        |                                                                                                                                                                                                                                                                                                 |
|--------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| amount | Number of satoshis (0.00000001 BTC) to be paid                                                                                                                                                                                                                                                  |
| script | a "TxOut" script where payment should be sent. This will normally be one of the standard Bitcoin transaction scripts (e.g. pubkey OP\_CHECKSIG). This is optional to enable future extensions to this protocol that derive Outputs from a master public key and the PaymentRequest data itself. |

### PaymentDetails/PaymentRequest

Payment requests are split into two messages to support future
extensibility. The bulk of the information is contained in the
PaymentDetails message. It is wrapped inside a PaymentRequest message,
which contains meta-information about the merchant and a digital
signature.

        message PaymentDetails {
            optional string network = 1 [default = "main"];
            repeated Output outputs = 2;
            required uint64 time = 3;
            optional uint64 expires = 4;
            optional string memo = 5;
            optional string payment_url = 6;
            optional bytes merchant_data = 7;
        }

|                |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| network        | either "main" for payments on the production Bitcoin network, or "test" for payments on test network. If a client receives a PaymentRequest for a network it does not support it must reject the request.                                                                                                                                                                                                                                                                             |
| outputs        | one or more outputs where Bitcoins are to be sent. If the sum of outputs.amount is zero, the customer will be asked how much to pay, and the bitcoin client may choose any or all of the Outputs (if there are more than one) for payment. If the sum of outputs.amount is non-zero, then the customer will be asked to pay the sum, and the payment shall be split among the Outputs with non-zero amounts (if there are more than one; Outputs with zero amounts shall be ignored). |
| time           | Unix timestamp (seconds since 1-Jan-1970 UTC) when the PaymentRequest was created.                                                                                                                                                                                                                                                                                                                                                                                                    |
| expires        | Unix timestamp (UTC) after which the PaymentRequest should be considered invalid.                                                                                                                                                                                                                                                                                                                                                                                                     |
| memo           | UTF-8 encoded, plain-text (no formatting) note that should be displayed to the customer, explaining what this PaymentRequest is for.                                                                                                                                                                                                                                                                                                                                                  |
| payment\_url   | Secure (usually https) location where a Payment message (see below) may be sent to obtain a PaymentACK.                                                                                                                                                                                                                                                                                                                                                                               |
| merchant\_data | Arbitrary data that may be used by the merchant to identify the PaymentRequest. May be omitted if the merchant does not need to associate Payments with PaymentRequest or if they associate each PaymentRequest with a separate payment address.                                                                                                                                                                                                                                      |

The payment\_url specified in the PaymentDetails should remain valid at
least until the PaymentDetails expires (or as long as possible if the
PaymentDetails does not expire). Note that this is irrespective of any
state change in the underlying payment request; for example cancellation
of an order should not invalidate the payment\_url, as it is important
that the merchant's server can record mis-payments in order to refund
the payment.

A PaymentRequest is PaymentDetails optionally tied to a merchant's
identity:

        message PaymentRequest {
            optional uint32 payment_details_version = 1 [default = 1];
            optional string pki_type = 2 [default = "none"];
            optional bytes pki_data = 3;
            required bytes serialized_payment_details = 4;
            optional bytes signature = 5;
        }

|                              |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| payment\_details\_version    | See below for a discussion of versioning/upgrading.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| pki\_type                    | public-key infrastructure (PKI) system being used to identify the merchant. All implementation should support "none", "x509+sha256" and "x509+sha1".                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| pki\_data                    | PKI-system data that identifies the merchant and can be used to create a digital signature. In the case of X.509 certificates, pki\_data contains one or more X.509 certificates (see Certificates section below).                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| serialized\_payment\_details | A protocol-buffer serialized PaymentDetails message.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| signature                    | digital signature over a hash of the protocol buffer serialized variation of the PaymentRequest message, with all serialized fields serialized in numerical order (all current protocol buffer implementations serialize fields in numerical order) and signed using the private key that corresponds to the public key in pki\_data. Optional fields that are not set are not serialized (however, setting a field to its default value will cause it to be serialized and will affect the signature). Before serialization, the signature field must be set to an empty value so that the field is included in the signed PaymentRequest hash but contains no data. |

When a Bitcoin wallet application receives a PaymentRequest, it must
authorize payment by doing the following:

1.  Validate the merchant's identity and signature using the PKI system,
    if the pki\_type is not "none".
2.  Validate that customer's system unix time (UTC) is before
    PaymentDetails.expires. If it is not, then the payment request must
    be rejected.
3.  Display the merchant's identity and ask the customer if they would
    like to submit payment (e.g. display the "Common Name" in the first
    X.509 certificate).

PaymentRequest messages larger than 50,000 bytes should be rejected by
the wallet application, to mitigate denial-of-service attacks.

### Payment

Payment messages are sent after the customer has authorized payment:

        message Payment {
            optional bytes merchant_data = 1;
            repeated bytes transactions = 2;
            repeated Output refund_to = 3;
            optional string memo = 4;
        }

|                |                                                                                                                                                                                                                                                                                                     |
|----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| merchant\_data | copied from PaymentDetails.merchant\_data. Merchants may use invoice numbers or any other data they require to match Payments to PaymentRequests. Note that malicious clients may modify the merchant\_data, so should be authenticated in some way (for example, signed with a merchant-only key). |
| transactions   | One or more valid, signed Bitcoin transactions that fully pay the PaymentRequest                                                                                                                                                                                                                    |
| refund\_to     | One or more outputs where the merchant may return funds, if necessary. The merchant may return funds using these outputs for up to 2 months after the time of the payment request. After that time has expired, parties must negotiate if returning of funds becomes necessary.                     |
| memo           | UTF-8 encoded, plain-text note from the customer to the merchant.                                                                                                                                                                                                                                   |

If the customer authorizes payment, then the Bitcoin client:

1.  Creates and signs one or more transactions that satisfy (pay in
    full) PaymentDetails.outputs
2.  Validate that customer's system unix time (UTC) is still before
    PaymentDetails.expires. If it is not, the payment should be
    cancelled.
3.  Broadcast the transactions on the Bitcoin p2p network.
4.  If PaymentDetails.payment\_url is specified, POST a Payment message
    to that URL. The Payment message is serialized and sent as the body
    of the POST request.

Errors communicating with the payment\_url server should be communicated
to the user. In the scenario where the merchant's server receives
multiple identical Payment messages for an individual PaymentRequest, it
must acknowledge each. The second and further PaymentACK messages sent
from the merchant's server may vary by memo field to indicate current
state of the Payment (for example number of confirmations seen on the
network). This is required in order to ensure that in case of a
transport level failure during transmission, recovery is possible by the
Bitcoin client re-sending the Payment message.

PaymentDetails.payment\_url should be secure against man-in-the-middle
attacks that might alter Payment.refund\_to (if using HTTP, it must be
TLS-protected).

Wallet software sending Payment messages via HTTP must set appropriate
Content-Type and Accept headers, as specified in BIP 71:

    Content-Type: application/bitcoin-payment
    Accept: application/bitcoin-paymentack

When the merchant's server receives the Payment message, it must
determine whether or not the transactions satisfy conditions of payment.
If and only if they do, it should broadcast the transaction(s) on the
Bitcoin p2p network.

Payment messages larger than 50,000 bytes should be rejected by the
merchant's server, to mitigate denial-of-service attacks.

### PaymentACK

PaymentACK is the final message in the payment protocol; it is sent from
the merchant's server to the bitcoin wallet in response to a Payment
message:

        message PaymentACK {
            required Payment payment = 1;
            optional string memo = 2;
        }

|         |                                                                                                                                                                         |
|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| payment | Copy of the Payment message that triggered this PaymentACK. Clients may ignore this if they implement another way of associating Payments with PaymentACKs.             |
| memo    | UTF-8 encoded note that should be displayed to the customer giving the status of the transaction (e.g. "Payment of 1 BTC for eleven tribbles accepted for processing.") |

PaymentACK messages larger than 60,000 bytes should be rejected by the
wallet application, to mitigate denial-of-service attacks. This is
larger than the limits on Payment and PaymentRequest messages as
PaymentACK contains a full Payment message within it.

## Localization

Merchants that support multiple languages should generate
language-specific PaymentRequests, and either associate the language
with the request or embed a language tag in the request's
merchant\_data. They should also generate a language-specific PaymentACK
based on the original request.

For example: A greek-speaking customer browsing the Greek version of a
merchant's website clicks on a "Αγορά τώρα" link, which generates a
PaymentRequest with merchant\_data set to "lang=el&basketId=11252". The
customer pays, their bitcoin client sends a Payment message, and the
merchant's website responds with PaymentACK.message "σας ευχαριστούμε".

## Certificates

The default PKI system is X.509 certificates (the same system used to
authenticate web servers). The format of pki\_data when pki\_type is
"x509+sha256" or "x509+sha1" is a protocol-buffer-encoded certificate
chain:

        message X509Certificates {
            repeated bytes certificate = 1;
        }

If pki\_type is "x509+sha256", then the PaymentRequest message is hashed
using the SHA256 algorithm to produce the message digest that is signed.
If pki\_type is "x509+sha1", then the SHA1 algorithm is used.

Each certificate is a DER \[ITU.X690.1994\] PKIX certificate value. The
certificate containing the public key of the entity that digitally
signed the PaymentRequest must be the first certificate. This MUST be
followed by additional certificates, with each subsequent certificate
being the one used to certify the previous one, up to (but not
including) a trusted root authority. The trusted root authority MAY be
included. The recipient must verify the certificate chain according to
\[RFC5280\] and reject the PaymentRequest if any validation failure
occurs.

Trusted root certificates may be obtained from the operating system; if
validation is done on a device without an operating system, the [Mozilla
root
store](http://www.mozilla.org/projects/security/certs/included/index.html)
is recommended.

## Extensibility

The protocol buffers serialization format is designed to be extensible.
In particular, new, optional fields can be added to a message and will
be ignored (but saved/re-transmitted) by old implementations.

PaymentDetails messages may be extended with new optional fields and
still be considered "version 1." Old implementations will be able to
validate signatures against PaymentRequests containing the new fields,
but (obviously) will not be able to display whatever information is
contained in the new, optional fields to the user.

If it becomes necessary at some point in the future for merchants to
produce PaymentRequest messages that are accepted \*only\* by new
implementations, they can do so by defining a new PaymentDetails message
with version=2. Old implementations should let the user know that they
need to upgrade their software when they get an up-version
PaymentDetails message.

Implementations that need to extend messages in this specification shall
use tags starting at 1000, and shall update the [extensions
page](bip-0070/extensions.mediawiki "wikilink") via pull-req to avoid
conflicts with other extensions.

## References

[BIP 0071](bip-0071.mediawiki "wikilink") : Payment Protocol mime types

[BIP 0072](bip-0072.mediawiki "wikilink") : Payment Protocol bitcoin:
URI extensions

Public-Key Infrastructure (X.509) working group :
<http://datatracker.ietf.org/wg/pkix/charter/>

Protocol Buffers : <https://developers.google.com/protocol-buffers/>

## Reference implementation

Create Payment Request generator :
<https://bitcoincore.org/~gavin/createpaymentrequest.php>
([source](https://github.com/gavinandresen/paymentrequest "wikilink"))

BitcoinJ : <https://bitcoinj.github.io/payment-protocol#introduction>

## See Also

Javascript Object Signing and Encryption working group :
<http://datatracker.ietf.org/wg/jose/>

Wikipedia's page on Invoices: <http://en.wikipedia.org/wiki/Invoice>
especially the list of Electronic Invoice standards

sipa's payment protocol proposal: <https://gist.github.com/1237788>

ThomasV's "Signed Aliases" proposal :
<http://ecdsa.org/bitcoin_URIs.html>

Homomorphic Payment Addresses and the Pay-to-Contract Protocol :
<http://arxiv.org/abs/1212.3257>
