+++
title = "Block size increase to 2MB"
date = 2015-06-23
weight = 102
in_search_index = true

[taxonomies]
authors = ["Jeff Garzik"]
status = ["Rejected"]

[extra]
bip = 102
status = ["Rejected"]
github = "https://github.com/bitcoin/bips/blob/master/bip-0102.mediawiki"
+++

      BIP: 102
      Layer: Consensus (hard fork)
      Title: Block size increase to 2MB
      Author: Jeff Garzik <jgarzik@gmail.com>
      Comments-Summary: No comments yet.
      Comments-URI: https://github.com/bitcoin/bips/wiki/Comments:BIP-0102
      Status: Rejected
      Type: Standards Track
      Created: 2015-06-23

## Abstract

Simple, one-time increase in total amount of transaction data permitted
in a block from 1MB to 2MB.

## Motivation

1.  Continue current economic policy.
2.  Exercise hard fork network upgrade.

## Specification

1.  MAX_BLOCK_SIZE increased to 2,000,000 bytes at trigger point.
2.  Increase maximum block sigops by similar factor, preserving SIZE/50
    formula.
3.  Trigger: (1) Block time 00:00:00 on flag day, AND (2) 95% of the
    last 1,000 blocks have signaled support.

## Backward compatibility

Fully validating older clients are not compatible with this change. The
first block exceeding 1,000,000 bytes will partition older clients off
the new network.

## Discussion

In the short term, an increase is needed to continue to current economic
policies with regards to fees and block space, matching market
expectations and preventing market disruption.

In the long term, this limit should focus on reflecting the maximum
network engineering limit.

## Implementation

<https://github.com/jgarzik/bitcoin/tree/2015_2mb_blocksize>
