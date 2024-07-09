with open('input', 'r') as f:
    entries = [int(line) for line in f.readlines()]


# Part 1
complements = set()
for e in entries:
    if e not in complements :
        partner = 2020 - e
        complements.add(partner)
    else :
        duo = (e, 2020-e)
        break

prod = duo[0] * duo[1]
print(f"Part 1. Product is {prod}")

# Part 2
def tripple():
    L = len(entries)
    for i in range(L):
        for j in range(i+1,L):
            for k in range(j+1,L):
                if entries[i] + entries[j] + entries[k] == 2020:
                    return (entries[i], entries[j], entries[k])

tri = tripple()
prod = tri[0] * tri[1] * tri[2]
print(f"Part 2. Tripple {tri} has product {prod}")