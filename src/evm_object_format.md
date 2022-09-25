# EVM Object Format

The general EVM Object Format as well as the first introduced version is
proposed in [EIP-3540](https://eips.ethereum.org/EIPS/eip-3540), currently
Considered For Inclussion in the [Shanghai Network
Update](https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/shanghai.md).

In this EIP an extensible and versioned contained format for the EVM (allowing
easy introduction of [variety of changes in the
future](./eof_benefits.md#a-non-exhaustive-list-of-proposed-changes-which-could-benefit-from-this-format)),
with once-off validation at **deploy time**.

## EOF Format Description

The container starts with the [EOF header](#eof-header), followed by at least
one [section header](#sections).

### EOF Header

EVM code following the EVM Object format have the following structure:

| Description| Length | Value        |                    |
|------------|--------|--------------|--------------------|
| magic      | 2-bytes| `0xEF00`     |                    |
| version    | 1-byte | `0x01`–`0xFF`| EOF version number |

#### Magic

1. The first byte `0xEF` was chosen because it is reserved for this purpose by
   [EIP-3541](https://eips.ethereum.org/EIPS/eip-3541) already deployed in
   [London Network Update](https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/london.md).
1. The second byte `0x00` was chosen to avoid clashes with the three contracts
   which were deployed on **Mainnet**:
   - [`EFF09f918bf09f9fa9`](https://etherscan.io/address/0xca7bf67ab492b49806e24b6e2e4ec105183caa01)
   - [`EF`](https://etherscan.io/address/0x897da0f23ccc5e939ec7a53032c5e80fd1a947ec)
   - [`EF`](https://etherscan.io/address/0x6e51d4d9be52b623a3d3a2fa8d3c5e3e01175cd0)
1. No contracts starting with `0xEF` bytes exist on public testnets: Goerli,
   Ropsten, Rinkeby, Kovan and Sepolia at their London fork block.

#### Version

EOF verstion range starts with 1. The version number 0 will never be used in
EOF so we can call legacy code *EOF0*. Also implementations may use API where
0 version number denotes legacy code.

### Sections

Each section header contains two fields, `section_kind` and `section_size`:

|Description   |Length |Value             |                                              |
|--------------|-------|------------------|----------------------------------------------|
|`section_kind`|1-byte | `0x01`-`0xff`    |Encoded as a 8-bit unsigned number            |
|`section_size`|2-bytes| `0x0001`-`0xffff`|Encoded as a 16-bit unsigned big-endian number|

## EOF Format Validation

When the specified block introducing the new Network Update is reached, code
validation will be introduced at contract creation for contracts following the
EVM Object Format.

If *[initcode](./glossary.md#initcode)* or *[code](./glossary.md#code)* starts
with the [`magic`](#eof-header), it is considered to be EOF formatted and will
undergo the following validations:

1. [`version`](#eof-header) MUST NOT be `0`.
1. [`section_kind`](#sections) MUST NOT be `0`. The value `0` is reserved for
    *section headers terminator* byte.
1. [`section_size`](#sections) MUST NOT be `0`. If a section is empty it section
    header MUST be omitted.
1. There MUST be at least one section (and therefore section header).
1. Section data MUST be equal to `section_size` declared in its header.
1. Stray bytes outside of sections MUST NOT be present. This includes trailing
   bytes after the last section.

Currently the only valid version is version `1`, any other version is invalid.

## Changes to contract creation semantics

1. If *[initcode](./glossary.md#initcode)* has EOFn prefix it MUST be valid
   EOFn code for version *n*.
1. If *[code](./glossary.md#code)* has EOFn prefix it MUST be valid EOF code
   for version *n*.

## Contract Creation Restrictions

Since *[initcode](./glossary.md#initcode)* and *[code](./glossary.md#code)* are
evaluated for EOF1 independently, the following combinations are allowed:

- Create transaction with EOFn *initcode* can deploy legacy contract
- Create transaction with legacy *initcode* can deploy EOFn contract
- EOFn contract can execute `CREATE` instruction with legacy *initcode* to
    create a new legacy contract
- Legacy contract can execute `CREATE` instruction with EOFn *initcode* to
    create a new EOFn contract
- Legacy contract can execute `CREATE` instruction with EOFn *initcode* to
    create a new legacy contract

To limit the number of exotic bytecode version combination, additional
restrictions are considered, but currently are not part of the specification:

1. The EOF version of *initcode* must much the version of *code*.
1. An EOF contract must not create legacy contracts.


