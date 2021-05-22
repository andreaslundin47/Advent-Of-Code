
with open('input', 'r') as f:
    lines = [li.strip() for li in f.readlines()]

    pa = {}
    passports = [pa]
    for line in lines:
        if line == '':
            pa = {}
            passports.append(pa)
        else:
            for entry in line.split():
                k, v = entry.split(':')
                pa[k] = v

# Part 1
valid = len([p for p in passports if len(p) == 8 or (len(p) == 7 and 'cid' not in p)])
print(f"Part 1. Valid passports: {valid}")

# Part 2
mandatory_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
eye_colours = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
valid = 0
for passport in passports:
    if not all([mf in passport for mf in mandatory_fields]):
        continue

    try:
        if not (1920 <= int(passport['byr']) <= 2002):
            continue

        if not (2010 <= int(passport['iyr']) <= 2020):
            continue

        if not (2020 <= int(passport['eyr']) <= 2030):
            continue

        height = passport['hgt']
        value, unit = int(height[:-2]), height[-2:]
        if not ((unit == 'cm' and 150 <= value <= 193) or (unit == 'in' and 59 <= value <= 76)):
            continue

        hcl = passport['hcl']
        if hcl[0] != '#' or len(hcl) != 7:
            continue
        for c in hcl[1:]:
            if c not in '0123456789abcdef':
                continue

        if passport['ecl'] not in eye_colours:
            continue

        pid = passport['pid']
        if len(pid) != 9:
            continue
        if not 0 <= int(pid) < 10**9:
            continue

        valid += 1
    except :
        continue


print(f"Part 2. Valid passports: {valid}")