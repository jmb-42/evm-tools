import pandas as pd


fname = "./evm-opcode-gas-costs/opcode-gas-costs_EIP-150_revision-1e18248_2017-04-12.csv"
with open(fname, "r") as f:
    df = pd.read_csv(fname)
df = df.fillna("")

fname = "./opcodes_template.rs"
with open(fname, "r") as f:
    template_lines = f.readlines()
num_lines = len(template_lines)
for i in range(num_lines-1, 0, -1):
    if template_lines[i].startswith('    //'):
        template_lines.pop(i)

template_code = ""
for i in range(len(df)):
    value = df.at[i, "Value"]
    mnemonic = df.at[i, "Mnemonic"]
    gas = df.at[i, "Gas Used"]
    subset = df.at[i, "Subset"]
    delta = df.at[i, "Removed from stack"]
    alpha = df.at[i, "Added to stack"]
    notes = df.at[i, "Notes"]
    formula_notes = df.at[i, "Formula Notes"]

    try:
        template_code += """
    let value = %i;
    let gas = %i;
    let delta = %i;
    let alpha = %i;
    let mnemonic = "%s";
    let subset = "%s";
    let notes = "%s";
    let formula_notes = "%s";
    let opcode_%s = Opcode{
        value, gas, delta, alpha,
        mnemonic, subset, notes, formula_notes
    };
    println!("{:?}", opcode_%s);
""" % (
            # assignments
            int(value, 16),
            int(gas),
            int(delta),
            int(alpha),
            mnemonic,
	        subset,
            notes,
            formula_notes,
            # identifiers
            mnemonic.lower(),
            mnemonic.lower()
            )
    except:
        # ignore opcodes with dynamic gas costs and ignore PUSH/DUP/SWAP
        continue

with open('./opcodes/src/main.rs', 'w+') as f:
    f.write(''.join([
        *template_lines[:-1], template_code, "}"
    ]))
