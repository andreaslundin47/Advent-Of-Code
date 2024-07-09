with open('input', 'r') as f:
    groups = f.read().strip().split('\n\n')

def yes_in_group(group):
    yeses = set()
    for line in group.split('\n'):
        for c in line:
            yeses.add(c)
    return len(yeses)

def all_yes_in_group(group):
    answers = [set(answers) for answers in group.split('\n')]
    inter = answers[0].copy()
    for ans in answers[1:]:
        inter.intersection_update(ans)
    return len(inter)


# Part 1
s = sum([yes_in_group(g) for g in groups])
print(f"Part 1. Yes counts: {s}")

# Part 2
s = sum([all_yes_in_group(g) for g in groups])
print(f"Part 2. Yes counts: {s}")