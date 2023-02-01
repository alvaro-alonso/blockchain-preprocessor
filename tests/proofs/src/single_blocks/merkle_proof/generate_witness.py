import hashlib
import json


def sha256(preimage: bytearray):
    return hashlib.sha256(preimage).digest()

def sha256_to_u32_array8(val: bytearray, abi=False) -> list:
    array_arrays = [val[i:i + 4] for i in range(0, len(val), 4)]
    u32_array = [str(int.from_bytes(x, 'big', signed=False)) for x in array_arrays]
    if abi:
        u32_array = ["0x" + str(int(x, 16)) for x in u32_array]
    return u32_array

def witness_cli_args(root: bytearray, leaf: bytearray, direction_selector: [bool], path: [bytearray]):
    formatted_root = " ".join(sha256_to_u32_array8(root))
    formatted_leaf = " ".join(sha256_to_u32_array8(leaf))
    formatted_dirselector = " ".join([str(int(x)) for x in direction_selector])
    formatted_path = " ".join([" ".join(sha256_to_u32_array8(x)) for x in path])
    return f"{formatted_root} {formatted_leaf} {formatted_dirselector} {formatted_path}"

if __name__ == "__main__":

    accounts = [ 
        int(x, base=16).to_bytes(32, "big", signed=False) for x in [
            "92b34907ec85874ab6faadaa1fd32b3e86c1211c8fb9350a8bf13bd5caf1ff29",
            "5d18f5dbfb0469a7136a3f3f95f46b3b1dbf434e1f9d95574ed8d40f8a40d0f1",
            "ad8dfd7d9441a9c44a796a319cf9a9cc80b056e06b7a3923cc20ce0c8ce1c4a9",
            "2b0a27e9cf8c514a1f01f7b905d6646b066b86a40f87c8fad4277c35cf85eec2",
            "369850511d58b5a40f57279d8ef1e18f84bfed2884ee463307a67077e4c75bf2",
            "9f762e459a5c3c6130ddaa04efa3d9aaf29424e73cb079b4f99a651332b8935a",
            "6760fd62081bbc67ae0320d61122e54871d026be124f41e11df2336077a93c39",
            "5bd9416d7cb4ff4560398e9d6b242fc3aaccea9c456038dcfb548a9ad8593945",
        ]
    ]

    h0 = sha256(accounts[0] + accounts[1])
    h1 = sha256(accounts[2] + accounts[3])
    h2 = sha256(accounts[4] + accounts[5])
    h3 = sha256(accounts[6] + accounts[7])

    h00 = sha256(h0 + h1)
    h01 = sha256(h2 + h3)

    root = sha256(h00 + h01)

    dir_sel = [True, False, False]

    path = [accounts[0], h1, h01]

<<<<<<< HEAD
    obj = json.dumps([
        sha256_to_u32_array8(root),
        {
            "leaf": sha256_to_u32_array8(accounts[1]),
            "directionSelector": dir_sel,
            "path": [sha256_to_u32_array8(x) for x in path],
        }
    ], indent=4)

    with open("sample.json", "w") as outfile:
        outfile.write(obj)
=======
    print(witness_cli_args(root, accounts[1], dir_sel, path))
>>>>>>> 41-prepare-test-data-and-pumping-script
