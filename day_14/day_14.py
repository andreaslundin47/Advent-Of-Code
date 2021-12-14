from collections import defaultdict

with open('input', 'r') as f :
    starting_sequence, raw_rules = f.read().split('\n\n')
    rules = {}
    for rule in raw_rules.strip().split('\n') :
        parts, product = rule.split(' -> ')
        rules[parts] = product


def polymerization(initial_sequence, rules, steps) :
    letter_counts = defaultdict(int)
    for c in initial_sequence :
        letter_counts[c] += 1

    pairs = defaultdict(int)
    for i in range(len(initial_sequence)-1) :
        pairs[initial_sequence[i:i+2]] += 1
    
    for _ in range(steps) :
        new_pairs = defaultdict(int)
        for pair, number in pairs.items() :
            product = rules[pair]
            left, right = pair
            letter_counts[product] += number
            new_pairs[f"{left}{product}"] += number
            new_pairs[f"{product}{right}"] += number
        pairs = new_pairs

    return max(letter_counts.values()) - min(letter_counts.values())


#### Part 1
diff = polymerization(starting_sequence, rules, 10)
print(f"Part 1. Difference: {diff}")

#### Part 2
diff = polymerization(starting_sequence, rules, 40)
print(f"Part 2. Difference: {diff}")