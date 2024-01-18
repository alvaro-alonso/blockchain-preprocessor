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

msg = (5).to_bytes(64, "big", signed=False)

u32 = sha256_to_u32_array8(msg)

root = sha256_to_u32_array8(sha256(msg))
print()

thousand = json.dumps([[root, [u32[:8], u32[8:]]] for _ in range(1000)], indent=4)

with open("sample.json", "w") as outfile:
        outfile.write(thousand)

