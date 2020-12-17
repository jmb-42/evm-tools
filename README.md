## Overview
This repo will evolve into a suite of evm tools for the command line. Initially, I will build toward an assembler/compiler, but I want to add more interesting features later.

## Motivation
Learn rust by starting with a basic, "zero-feature" EVM compiler.

## Usage
For now, the only functionality of this repo is to print the each of the implemented rust Opcode structs.
```sh
git clone --recursive https://github.com/jmb-42/evm-tools
cd evm-tools/opcodes
cargo run
```

## Goals
1. Initial implementation should compile assembly sourcecode to evm bytecode with the assembly source having the following properties:
 - represents opcodes as string mnemonics
   - e.g. `PUSH1`, `MULMOD`, `CALLDATALOAD`, etc.
 - prefixes hex literals with `0x`
   - e.g. `0x01`, `0x0a10e4`, etc.
 - translates integer literals directly for uints
   - does not allow for signed integers
 - each line in a file represents a single opcode instruction and its arguments
   - lines are read right-to-left
     - e.g. `0x01 PUSH` to push 1 onto the stack
   - files are read top-to-bottom
 - implement 256-bit types
2. Future implementations should:
 - incorporate JUMPDEST labels
 - validate opcode stack inputs and stack outputs

## Progress
#### skeleton of rust opcodes library
I produced the current rust code by processing the [evm-opcode-gas-costs](https://github.com/djrtwo/evm-opcode-gas-costs) csv file in Python. The Python script used a simple rust template that I wrote to hardcode the current opcodes as instances of a rust struct. As a result, opcodes/src/main.rs is incomplete:
 - opcodes with formulaic gas costs are not included yet
   - opcode structs with generic gas funcs/constants aren't implemented yet
 - The PUSHX, DUPX, and SWAPX opcodes are not yet included

The accuracy of the table hasn't been scrutinized, and this code does nothing meaningful yet. Although I used Python to generate this early definitions file, I do not intend to continue this practice. In hindsight, maybe I should have used rust to generate the rust code...

## Crates
Here is a working list of crates I may use for this project.

#### Parsers
 - [nom](https://crates.io/crates/nom)
 - [regex](https://crates.io/crates/regex)

#### bigints/biguints
 - [num-bigint](https://crates.io/crates/num-bigint)
 - [num-bigint-dig](https://crates.io/crates/num-bigint-dig)
 - [uint](https://crates.io/crates/uint)


## Reproduce the rust code using python
You can recreate the `opcodes` rust project directory on your own unix-like system. You need python3 and rust, and then follow these terminal commands:
```sh
git clone --recursive https://github.com/jmb-42/evm-tools
cd evm-tools
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python opcodes_to_rust.py
```
Test that it works:
```sh
cd opcodes
cargo run
```