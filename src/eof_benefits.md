# EOF Benefits

## Code and Data Separation

This separation is especially benefical for on-chain code validators (like
those utilised by layer-2 scaling solutions, such as Optimism), because they
can distinguish code and data (this includes deployment code and constructor
arguments too).

Currently they:

- a) Require changes prior to contract deployment
- b) Implement a fragile method
- c) Implement and expensive and restrictive code jump analysis.

Code and data separation can result in ease of use and significant gas savings
for such cases.

## A non-exhaustive list of proposed changes which could benefit from this format:

- Including a `JUMPDEST`-table (to avoid analysis at exection time) and/or
    removing `JUMPDEST`s entirely.
- Introducing static jups (with relative addresses) and jump tables, and
    disallowing dynamic jumps at the same time.
- Requiring code section(s) to be terminated by `STOP`. (Assumptions like this
    can provide significant speed improvements, such as a speed up of ~7% seen
    in [evmone](https://github.com/ethereum/evmone/pull/295).
- Multi-byte opcodes without any workarounds.
- Representing functions as individual code sections instead of subroutines.
- Introducing special sections for different use cases, notably Account
    Abstraction.


