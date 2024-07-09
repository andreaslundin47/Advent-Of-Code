from statistics import median

with open('input', 'r') as f :
    code_strings = [line.strip() for line in f.readlines()]

openers = set( ['(', '[', '{', '<'] )
matching_closer = { '(':')', '[':']', '{':'}', '<':'>' }
error_points = {')':3, ']':57, '}':1197, '>':25137}
completion_points = {')':1, ']':2, '}':3, '>':4}


error_score = 0
completion_scores = []

for code in code_strings :
    stack = []
    erronious_string = False
    for c in code :
        if c in openers :
            stack.append(c)
        elif matching_closer[stack[-1]] == c :
            stack.pop()
        else :
            error_score += error_points[c]
            erronious_string = True
            break
    if not erronious_string :
        score = 0
        for c in reversed(stack) :
            score = 5 * score + completion_points[matching_closer[c]]
        completion_scores.append(score)

completion_score = median(completion_scores)

print(f"Part 1. Total Syntax Error Score: {error_score:15}")
print(f"Part 2. Middle Completion Score:  {completion_score:15}")