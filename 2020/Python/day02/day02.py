with open('input', 'r') as f:
    entries = []
    for row in f.readlines():
        rule, pw = row.split(':')
        rang, letter = rule.strip().split()
        lower, upper = rang.split('-')
        password = pw.strip()
        entry = (int(lower), int(upper), letter, password)
        entries.append(entry)

        
# Part 1
valid = len([pw for L,U,ch,pw in entries if L <= pw.count(ch) <= U])
print(f"Part 1. Valid passwords: {valid}")

# Part 2
valid = len([pw for L,U,ch,pw in entries if [pw[L-1], pw[U-1]].count(ch) == 1])
print(f"Part 2. Valid passwords: {valid}")