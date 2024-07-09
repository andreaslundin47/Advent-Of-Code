
# Parsing
with open('input', 'r') as f:
    rules = []
    for line in f.readlines():
        inner_bag, others = line.split('bags contain')
        inner_ID = inner_bag.strip()
        rule = (inner_ID, set())
        if 'no other' not in others:
            outers = others.split(',')
            for outer in outers:
                count, n1, n2, b = outer.strip().strip('.').split()
                rule[1].add( (int(count), f"{n1} {n2}") )
        rules.append(rule)

# For each bag, how many of the other bags does it have to contain?
bags_inside_of_bag = {}
for outer, contents in rules:
    bags_inside_of_bag[outer] = contents

# For each bag, a list with all the the bags that are allowed to contain that particular bag
allowed_to_include = {}
for container, inner_bags in rules:
    if container not in allowed_to_include:
        allowed_to_include[container] = set()
    for count, bag in inner_bags:
        if bag not in allowed_to_include:
            allowed_to_include[bag] = set()
        allowed_to_include[bag].add(container)

# Part 1 - Basically a BFS starting from 'shiny gold'
seen = allowed_to_include['shiny gold'].copy()
unexpanded = seen.copy()

while unexpanded:
    outer_bag = unexpanded.pop()
    for bag in allowed_to_include[outer_bag]:
        if bag not in seen and bag != 'shiny gold':
            seen.add(bag)
            unexpanded.add(bag)

ext = len(seen)
print(f"Part 1. Number of possible exterior bags: {ext}")

# Part 2 - Recursively sum up all the bags inside of 'shiny gold'

def number_of_bags_in_bag(outer):
    total = 0
    for count, inner_bag in bags_inside_of_bag[outer]:
        total += count * (1 + number_of_bags_in_bag(inner_bag))
    return total

bags = number_of_bags_in_bag('shiny gold')
print(f"Part 2. Total number of bags inside one 'Shiny Gold': {bags}")
