# Overview
This repo will evolve into a suite of evm tools for the command line. Initially, I will build toward an assembler/compiler, but I want to add more interesting features later.

## Motivation
Learn rust by starting with a basic, zero-feature EVM compiler.

# Progress
 - Take the [opcodes gas costs data](https://github.com/djrtwo/evm-opcode-gas-costs)
 - Use Python to convert it into valid rust code
   - Only static gas cost opcodes are included yet
   - The PUSHX, DUPX, and SWAPX opcodes need to be added

Although I used Python for this early definitions file, I will be writing rust code directly. Furthermore, the accuracy of the table hasn't been scrutinized, and this code does nothing yet (except for print the opcodes to the terminal).

## Crates
Here is a working list of crates I may use for this project.

Parsers

 - [nom](https://crates.io/crates/nom)
 - [regex](https://crates.io/crates/regex)

bigints/biguints

 - [num-bigint](https://crates.io/crates/num-bigint)
 - [num-bigint-dig](https://crates.io/crates/num-bigint-dig)
 - [uint](https://crates.io/crates/uint)

# Installation
To keep things interesting, I didn't upload the rust code produced by the Python script. To create it on your own unix-like system, you need python3 and rust:
```sh
git clone https://github.com/jmb-42/evm-tools
cd evm-tools
cargo new opcodes
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