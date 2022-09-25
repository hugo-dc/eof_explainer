# EOF Version 1

[EIP-3540](https://eips.ethereum.org/EIPS/eip-3540) allows [code and data
separation](./eof_benefits.md#code-and-data-separation), by introducing two
[section kinds](#section-kinds): code and data sections, as well as a sections
header separator..

## Section Kinds

| Section Kind | Meaning                                       |
|--------------|-----------------------------------------------|
| 0            | *reserved for section headers terminator byte*|
| 1            | code                                          |
| 2            | data                                          |

## EOF Version 1 Validation Rules

In addition to general validation rules [in the previous
page](./evm_object_format.md#eof-format-validation), EOF version 1 bytecode
conforms to the following rules:

- Exactly one code section MUST be present
- The code section MUST be the first section
- A single data section MAY follow the code section.

(*Remark*: Contract creation code SHOULD set the section size of the data so
that the constructor argument fit it.)

## Changes to execution semantics

For clarity, the *container* referst to the complete account EOF code, while
*code* refers to the contents of the code section only.

1. `JUMPDEST` analysis is only run on the *code*.
1. Execution starts at the first byte of the *code*, and `PC` is set to 0.
1. If `PC` goes outside of the code section bounds, execution aborts with
   failure.
1. `JUMP`/`JUMPI` uses an absolute offset within the *code*.
1. `CODECOPY`/`CODESIZE`/`EXTCODECOPY`/`EXTCODESIZE`/`EXTCODEHASH` keeps
   operating on the entire *container*.
1. The input to `CREATE`/`CREATE2` is still the entire *container*.

## Data-only contracts

For EOF Data-only contracts, which purpose is to store data and not execution,
**EOF1 requires** presence of a **code section** therefore the minimal overhead
EOF data contract consist of a data section and one code section with a single
instruction. We recommend to use `INVALID` (`0xfe`) instruction in this case. In total
there are 11 additional bytes required:

`EF0001 01<data size> 00 FE <data>`

> **note**: I think this should be: `EF0001 010001 02<data-size> 00 FE <data>`

## Test cases

You can modify and tests the folowing tests [here](./tests_eip_3540.md).

### Legacy contracts

```rust
{{#rustdoc_include code/eip_3540.rs:legacy_contracts}}
```

### Code without eof magic

```rust
{{#rustdoc_include code/eip_3540.rs:no_eof_magic}}
```

### EOF1 Container


```rust
{{#rustdoc_include code/eip_3540.rs:eof1_container}}
```

