#[derive(Debug)]
struct Opcode {
    value: u64,
    gas: u64,
    delta: u64,
    alpha: u64,
    mnemonic: &'static str,
    subset: &'static str,
    notes: &'static str,
    formula_notes: &'static str
}

fn main() {
    let value = 0;
    let gas = 0;
    let delta = 0;
    let alpha = 0;
    let mnemonic = "STOP";
    let subset = "zero";
    let notes = "Halts execution.";
    let formula_notes = "";
    let opcode_stop = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_stop);
    let value = 1;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "ADD";
    let subset = "verylow";
    let notes = "Addition operation";
    let formula_notes = "";
    let opcode_add = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_add);
    let value = 2;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "MUL";
    let subset = "low";
    let notes = "Multiplication operation.";
    let formula_notes = "";
    let opcode_mul = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mul);
    let value = 3;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SUB";
    let subset = "verylow";
    let notes = "Subtraction operation.";
    let formula_notes = "";
    let opcode_sub = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_sub);
    let value = 4;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "DIV";
    let subset = "low";
    let notes = "Integer division operation.";
    let formula_notes = "";
    let opcode_div = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_div);
    let value = 5;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SDIV";
    let subset = "low";
    let notes = "Signed integer division operation (truncated).";
    let formula_notes = "";
    let opcode_sdiv = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_sdiv);
    let value = 6;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "MOD";
    let subset = "low";
    let notes = "Modulo remainder operation";
    let formula_notes = "";
    let opcode_mod = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mod);
    let value = 7;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SMOD";
    let subset = "low";
    let notes = "Signed modulo remainder operation.";
    let formula_notes = "";
    let opcode_smod = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_smod);
    let value = 8;
    let gas = 8;
    let delta = 3;
    let alpha = 1;
    let mnemonic = "ADDMOD";
    let subset = "mid";
    let notes = "Modulo addition operation.";
    let formula_notes = "";
    let opcode_addmod = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_addmod);
    let value = 9;
    let gas = 8;
    let delta = 3;
    let alpha = 1;
    let mnemonic = "MULMOD";
    let subset = "mid";
    let notes = "Modulo multiplication operation.";
    let formula_notes = "";
    let opcode_mulmod = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mulmod);
    let value = 11;
    let gas = 5;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SIGNEXTEND";
    let subset = "low";
    let notes = "Extend length of two’s complement signed integer.";
    let formula_notes = "";
    let opcode_signextend = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_signextend);
    let value = 16;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "LT";
    let subset = "verylow";
    let notes = "Less-than comparison.";
    let formula_notes = "";
    let opcode_lt = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_lt);
    let value = 17;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "GT";
    let subset = "verylow";
    let notes = "Greater-than comparison.";
    let formula_notes = "";
    let opcode_gt = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_gt);
    let value = 18;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SLT";
    let subset = "verylow";
    let notes = "Signed less-than comparison.";
    let formula_notes = "";
    let opcode_slt = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_slt);
    let value = 19;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "SGT";
    let subset = "verylow";
    let notes = "Signed greater-than comparison.";
    let formula_notes = "";
    let opcode_sgt = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_sgt);
    let value = 20;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "EQ";
    let subset = "verylow";
    let notes = "Equality comparison.";
    let formula_notes = "";
    let opcode_eq = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_eq);
    let value = 21;
    let gas = 3;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "ISZERO";
    let subset = "verylow";
    let notes = "Simple not operator.";
    let formula_notes = "";
    let opcode_iszero = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_iszero);
    let value = 22;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "AND";
    let subset = "verylow";
    let notes = "Bitwise AND operation.";
    let formula_notes = "";
    let opcode_and = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_and);
    let value = 23;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "OR";
    let subset = "verylow";
    let notes = "Bitwise OR operation";
    let formula_notes = "";
    let opcode_or = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_or);
    let value = 24;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "XOR";
    let subset = "verylow";
    let notes = "Bitwise XOR operation.";
    let formula_notes = "";
    let opcode_xor = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_xor);
    let value = 25;
    let gas = 3;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "NOT";
    let subset = "verylow";
    let notes = "Bitwise NOT operation.";
    let formula_notes = "";
    let opcode_not = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_not);
    let value = 26;
    let gas = 3;
    let delta = 2;
    let alpha = 1;
    let mnemonic = "BYTE";
    let subset = "verylow";
    let notes = "Retrieve single byte from word";
    let formula_notes = "";
    let opcode_byte = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_byte);
    let value = 48;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "ADDRESS";
    let subset = "base";
    let notes = "Get address of currently executing account.";
    let formula_notes = "";
    let opcode_address = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_address);
    let value = 49;
    let gas = 400;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "BALANCE";
    let subset = "";
    let notes = "Get balance of the given account.";
    let formula_notes = "";
    let opcode_balance = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_balance);
    let value = 50;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "ORIGIN";
    let subset = "base";
    let notes = "Get execution origination address.";
    let formula_notes = "";
    let opcode_origin = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_origin);
    let value = 51;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "CALLER";
    let subset = "base";
    let notes = "Get caller address.";
    let formula_notes = "";
    let opcode_caller = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_caller);
    let value = 52;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "CALLVALUE";
    let subset = "base";
    let notes = "Get deposited value by the instruction/transaction responsible for this execution.";
    let formula_notes = "";
    let opcode_callvalue = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_callvalue);
    let value = 53;
    let gas = 3;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "CALLDATALOAD";
    let subset = "verylow";
    let notes = "Get input data of current environment.";
    let formula_notes = "";
    let opcode_calldataload = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_calldataload);
    let value = 54;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "CALLDATASIZE";
    let subset = "base";
    let notes = "Get size of input data in current environment.";
    let formula_notes = "";
    let opcode_calldatasize = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_calldatasize);
    let value = 56;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "CODESIZE";
    let subset = "base";
    let notes = "Get size of code running in current environment.";
    let formula_notes = "";
    let opcode_codesize = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_codesize);
    let value = 58;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "GASPRICE";
    let subset = "base";
    let notes = "Get price of gas in current environment.";
    let formula_notes = "";
    let opcode_gasprice = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_gasprice);
    let value = 59;
    let gas = 700;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "EXTCODESIZE";
    let subset = "extcode";
    let notes = "Get size of an account’s code.";
    let formula_notes = "";
    let opcode_extcodesize = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_extcodesize);
    let value = 64;
    let gas = 20;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "BLOCKHASH";
    let subset = "";
    let notes = "Get the hash of one of the 256 most recent complete blocks.";
    let formula_notes = "";
    let opcode_blockhash = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_blockhash);
    let value = 65;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "COINBASE";
    let subset = "base";
    let notes = "Get the block’s beneficiary address.";
    let formula_notes = "";
    let opcode_coinbase = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_coinbase);
    let value = 66;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "TIMESTAMP";
    let subset = "base";
    let notes = "Get the block’s timestamp.";
    let formula_notes = "";
    let opcode_timestamp = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_timestamp);
    let value = 67;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "NUMBER";
    let subset = "base";
    let notes = "Get the block’s number.";
    let formula_notes = "";
    let opcode_number = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_number);
    let value = 68;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "DIFFICULTY";
    let subset = "base";
    let notes = "Get the block’s difficulty.";
    let formula_notes = "";
    let opcode_difficulty = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_difficulty);
    let value = 69;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "GASLIMIT";
    let subset = "base";
    let notes = "Get the block’s gas limit.";
    let formula_notes = "";
    let opcode_gaslimit = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_gaslimit);
    let value = 80;
    let gas = 2;
    let delta = 1;
    let alpha = 0;
    let mnemonic = "POP";
    let subset = "base";
    let notes = "Remove item from stack.";
    let formula_notes = "";
    let opcode_pop = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_pop);
    let value = 81;
    let gas = 3;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "MLOAD";
    let subset = "verylow";
    let notes = "Load word from memory.";
    let formula_notes = "";
    let opcode_mload = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mload);
    let value = 82;
    let gas = 3;
    let delta = 2;
    let alpha = 0;
    let mnemonic = "MSTORE";
    let subset = "verylow";
    let notes = "Save word to memory";
    let formula_notes = "";
    let opcode_mstore = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mstore);
    let value = 83;
    let gas = 3;
    let delta = 2;
    let alpha = 0;
    let mnemonic = "MSTORE8";
    let subset = "verylow";
    let notes = "Save byte to memory.";
    let formula_notes = "";
    let opcode_mstore8 = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_mstore8);
    let value = 84;
    let gas = 200;
    let delta = 1;
    let alpha = 1;
    let mnemonic = "SLOAD";
    let subset = "";
    let notes = "Load word from storage";
    let formula_notes = "";
    let opcode_sload = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_sload);
    let value = 86;
    let gas = 8;
    let delta = 1;
    let alpha = 0;
    let mnemonic = "JUMP";
    let subset = "mid";
    let notes = "Alter the program counter";
    let formula_notes = "";
    let opcode_jump = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_jump);
    let value = 87;
    let gas = 10;
    let delta = 2;
    let alpha = 0;
    let mnemonic = "JUMPI";
    let subset = "high";
    let notes = "Conditionally alter the program counter.";
    let formula_notes = "";
    let opcode_jumpi = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_jumpi);
    let value = 88;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "PC";
    let subset = "base";
    let notes = "Get the value of the program counter prior to the increment corresponding to this instruction.";
    let formula_notes = "";
    let opcode_pc = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_pc);
    let value = 89;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "MSIZE";
    let subset = "base";
    let notes = "Get the size of active memory in bytes.";
    let formula_notes = "";
    let opcode_msize = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_msize);
    let value = 90;
    let gas = 2;
    let delta = 0;
    let alpha = 1;
    let mnemonic = "GAS";
    let subset = "base";
    let notes = "Get the amount of available gas, including the corresponding reduction for the cost of this instruction.";
    let formula_notes = "";
    let opcode_gas = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_gas);
    let value = 91;
    let gas = 1;
    let delta = 0;
    let alpha = 0;
    let mnemonic = "JUMPDEST";
    let subset = "";
    let notes = "Mark a valid destination for jumps";
    let formula_notes = "";
    let opcode_jumpdest = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_jumpdest);
    let value = 240;
    let gas = 32000;
    let delta = 3;
    let alpha = 1;
    let mnemonic = "CREATE";
    let subset = "";
    let notes = "Create a new account with associated code.";
    let formula_notes = "";
    let opcode_create = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_create);
    let value = 243;
    let gas = 0;
    let delta = 2;
    let alpha = 0;
    let mnemonic = "RETURN";
    let subset = "zero";
    let notes = "Halt execution returning output data.";
    let formula_notes = "";
    let opcode_return = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_return);
}