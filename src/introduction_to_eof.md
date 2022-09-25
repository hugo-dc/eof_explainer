# Introduction to EOF

## Motivation

Currently, on-chain deployed EVM bytecode contains no pre-defined structure.
Code is typically validated in clients to the extend of `JUMPDEST` analysis at
runtime, every single time prior to execution. This poses not only an
overhead, but also a challenge for introducing new or deprecating old
features.

Ethereum Object Formats allows validating code during contract creation,
allowing versioning without an additional version field in the account.
Versioning is a useful tool for introducing or deprecating features, especially
for larger changes (such as significant changes to control flow, or features
like Account Abstraction), without affecting existing contracts.

## Notes

- This document use [RFC2119](https://tools.ietf.org/html/rfc2119) keywords.
