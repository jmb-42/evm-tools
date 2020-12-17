import pandas as pd


# read in the opcode data
fname = "./evm-opcode-gas-costs/opcode-gas-costs_EIP-150_revision-1e18248_2017-04-12.csv"
with open(fname, "r") as f:
    df = pd.read_csv(fname)
# replace `nan` values with empty string
df = df.fillna("")

# read in the rust template
fname = "./opcodes_template.rs"
with open(fname, "r") as f:
    template_lines = f.readlines()
num_lines = len(template_lines)

# retrieve the template. process the list in reverse, so that `pop` doesn't shorten
# the length of the list before the `for` loop completes
template = []
for i in range(num_lines, 0, -1):
    line = template_lines[i-1]
    if line.startswith('    //'):
        template += [line.replace('// ', '')]
        template_lines.pop(i)
template = "".join(template[::-1])

# begin the writable code
rust_code = ""
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
        # ignore opcodes with dynamic gas costs and ignore PUSH/DUP/SWAP
        rust_code += template % (
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

final_rust_code = "".join([*template_lines[:-1], rust_code, "}"])

# write the code to the file
with open('./opcodes/src/main.rs', 'w+') as f:
    f.write(final_rust_code)
