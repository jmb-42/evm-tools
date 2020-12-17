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
    // let value = %i;
    // let gas = %i;
    // let delta = %i;
    // let alpha = %i;
    // let mnemonic = "%s";
    // let subset = "%s";
    // let notes = "%s";
    // let formula_notes = "%s";
    // let %s = Opcode{
    //     value, gas, delta, alpha,
    //     mnemonic, subset, notes, formula_notes
    // };
    // println!("{:?}", push)
}