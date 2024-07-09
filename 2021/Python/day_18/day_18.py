from itertools import permutations
import math
import json

with open('input', 'r') as f :
    snail_numbers = [line.strip() for line in f.readlines()]


def parse_string(number) :
    expr = []
    i = 0

    while i < len(number) :
        c = number[i]
        if c in set(['[', ']', ',']) :
            expr.append(c)
            i += 1
        elif c.isdigit() :
            num = ""
            while c.isdigit() :
                num += c
                i += 1
                c = number[i]
            expr.append(int(num))

    return expr
    

def explode(number) :
    bracket_count = 0
    for i, c in enumerate(number) :
        if c == '[' :
            bracket_count += 1
        elif c == ']' :
            bracket_count -= 1
        elif c == ',' and isinstance(number[i-1], int) and isinstance(number[i+1], int) and bracket_count > 4 :
            left, right = number[:i-2], number[i+3:]
            for j in range(len(left)-1, 0, -1) :
                if isinstance(left[j], int) :
                    left[j] += number[i-1]
                    break
            for k in range(0, len(right)) :
                if isinstance(right[k], int) :
                    right[k] += number[i+1]
                    break
            return True, left + [0] + right
    return False, number


def split(number) :
    for i, c in enumerate(number) :
        if isinstance(c, int) and c >= 10 :
            left, right = number[:i], number[i+1:]
            middle = ['[', math.floor(c/2), ',', math.ceil(c/2), ']']
            return True, left + middle + right
    return False, number


def reduce(number) :
    changed = True
    while changed :
        changed, number = explode(number)
        if not changed :
            changed, number = split(number)
    return number


def add(m, n) :
    if m is None :
        return n
    if n is None :
        return m 
    return reduce( ['['] + m + [','] + n + [']'] )


def magnitude(number) :
    def calc_mag(num) :
        if isinstance(num, int) :
            return num
        else :
            return 3 * calc_mag(num[0]) + 2 * calc_mag(num[1])

    s = "".join(str(v) if isinstance(v, int) else v for v in number)
    expr = json.loads(s)
    return calc_mag(expr)


#### Part 1

sum = None
for number in snail_numbers :
    sum = add(sum, parse_string(number))
mag = magnitude(sum)

print(f"Part 1. Magnitude: {mag}")


#### Part 2

max_magnitude = 0
for m, n in permutations(snail_numbers, r=2) :
    m, n = parse_string(m), parse_string(n)
    max_magnitude = max( max_magnitude, max( magnitude(add(m,n)), magnitude(add(n,m)) ) )
print(f"Part 2. Largest pairwise magnitude: {max_magnitude}")