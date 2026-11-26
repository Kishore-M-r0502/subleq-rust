# SUBLEQ Rust

A Rust implementation of a SUBLEQ (Subtract and Branch if Less than or Equal to Zero) machine.
[View on GitHub](https://github.com/Kishore-M-r0502/subleq-rust)

SUBLEQ Rust is an independently developed systems-programming project exploring One-Instruction Set Computers (OISCs), instruction execution, memory, parsing, and execution tracing.

## About

SUBLEQ is a minimalist computational model in which a single instruction performs subtraction and conditional branching.

The instruction has three operands:

```text
a b c
```

Its operation is:

```text
M[b] = M[b] - M[a]

if M[b] <= 0:
    PC = c
else:
    PC = PC + 3
```

SUBLEQ Rust implements this execution model using Rust.

## Current Features

- SUBLEQ instruction execution
- Configurable memory size
- Data and code sections
- External `.subleq` program loading
- Execution tracing
- Branching based on negative and zero results
- Special `-1` halt convention
- Input parsing and validation errors

## Requirements

- Rust toolchain
- Cargo

## Build

Clone the repository:

```bash
git clone https://github.com/Kishore-M-r0502/subleq-rust
cd subleq-rust
```

Build:

```bash
cargo build
```

Run the tests:

```bash
cargo test
```

## Running a Program

Provide a `.subleq` source file as a command-line argument. For example:

```bash
cargo run -- tests/programs/program.subleq
```

The executable accepts the input filename as its positional argument.

## Program Format

A SUBLEQ source file uses three main sections:

- `memory:` — optional memory-size declaration
- `data:` — data entries
- `code:` — SUBLEQ instructions

Example structure:

```text
memory: 256

data:
10 20 ;
11 30 ;
0 40 ;
end

code:
20 40 3 ;;
40 30 -1 ;;
end
```

Data entries use the format:

```text
value address ;
```

Instructions use the format:

```text
a b c ;;
```

Lines beginning with `#` are treated as comments.

## Memory and Execution

Memory is currently initialized to zero.

Each instruction occupies three memory cells. Execution begins at address zero.

The machine branches when the subtraction result is less than or equal to zero.

The current implementation also recognises `-1` as a special halt convention. This convention is applied after the subtraction operation.

## Example Programs

The `tests/programs/` directory contains example programs covering:

- Sequential execution
- Negative-result branching
- Zero-result branching
- Positive-result execution
- The special halt convention

It also contains the original example programs.

## Project Status

**Version:** 0.5.2

**Status:** Pre-release

This project is experimental. Its parser, memory model, execution safety, and builder design are still open to improvement.

Feedback, bug reports, design discussions, and contributions are welcome.

## License

SUBLEQ Rust is licensed under the GNU General Public License, version 3 or (at your option) any later version.

See [COPYING](COPYING) for the complete license text.

Copyright (C) 2026 Kishore M R
