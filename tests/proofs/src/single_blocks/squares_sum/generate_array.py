import json

obj = json.dumps([
   [str(x) for x in range(1000)] for _ in range(1000)
], indent=4)

with open("thousand_squares_sum.json", "w") as outfile:
    outfile.write(obj)
