import re

with open('input', 'r') as f:
    rules, messages = f.read().split('\n\n')
    rules = [rule.split(':') for rule in rules.split('\n')]
    rules = [(rule[0],rule[1].strip().strip('"')) for rule in rules]
    messages = [m.strip() for m in messages.strip().split('\n')]

scored_rules = {}
for parent, children in rules:
    parent = f"_{parent.strip()}_"
    if '|' in children:
        children = f"_(_{children.replace(' ', '_')}_)_"
    else:
        children = f"_{children.replace(' ', '_')}_"
    scored_rules[parent] = children


# Part 1
reg = scored_rules['_0_']
changed = True
while changed:
    temp = reg
    for pattern, replace_pattern in scored_rules.items():
        reg = reg.replace(pattern, replace_pattern, 1)
    changed = temp != reg
reg = reg.replace('_', '')
reg = f"^{reg}$"

pattern = re.compile(reg)

valid = [pattern.match(m) != None for m in messages]
print(f"Part 1. Valid messages: {sum(valid)}")

# Part 2

"""
    0 => 8 11 => 42+42{n}31{n}
"""

reg = scored_rules['_42_']
changed = True
while changed:
    temp = reg
    for pattern, replace_pattern in scored_rules.items():
        reg = reg.replace(pattern, replace_pattern, 1)
    changed = temp != reg
reg = reg.replace('_', '')
p42 = re.compile(reg)

reg = scored_rules['_31_']
changed = True
while changed:
    temp = reg
    for pattern, replace_pattern in scored_rules.items():
        reg = reg.replace(pattern, replace_pattern, 1)
    changed = temp != reg
reg = reg.replace('_', '')
p31 = re.compile(reg)

def validate(mes):
    if not p42.match(mes):
        return False        # Need to start with at least one 42 group
    idx = 0
    m42 = p42.match(mes)
    occurances42 = 0
    while m42:
        occurances42 += 1
        idx += len(m42.group(0))
        m42 = p42.match(mes[idx:])
    len_42 = idx 

    occurances31 = 0
    m31 = p31.match(mes[len_42:])
    while m31:
        occurances31 += 1
        idx += len(m31.group(0))
        m31 = p31.match(mes[idx:])

    if occurances31 == 0:
        return False        # Need at least one 31 group

    if idx != len(mes): # Sum of chars in all groups need to match message length
        return False
    if occurances42 <= occurances31: # Need at least one more 42 group than 31 group to satisfy rule
        return False
    return True
    
v = [validate(m) for m in messages]
#print(v)
print(f"Part 2. Valid messages: {sum(v)}")

