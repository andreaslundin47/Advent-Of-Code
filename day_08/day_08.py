from collections import Counter

with open('input', 'r') as f :
    cases = [line.strip() for line in f.readlines()]   

#### Part 1

count = 0
for case in cases :
    digits, output = case.split('|')
    numbers = [number.strip() for number in output.split()]
    for number in numbers :
        if len(number) in (2, 3, 4, 7) :
            count += 1

print(f"Part 1. Number of 1, 4, 7, and 8: {count}")

#### Part 2

"""
                     AAA         
                    B   C
                    B   C
                    B   C
    lowercase ->     DDD      ->  ACF => 7, etc.
                    E   F
                    E   F
                    E   F
                     GGG
"""

segment2number = {
                    'ABCEFG'  : 0,
                    'CF'      : 1,
                    'ACDEG'   : 2,
                    'ACDFG'   : 3,
                    'BCDF'    : 4,
                    'ABDFG'   : 5,
                    'ABDEFG'  : 6,
                    'ACF'     : 7,
                    'ABCDEFG' : 8,
                    'ABCDFG'  : 9
                 }

def make_translation(digits_string) :
    patterns = digits_string.strip().split()
    for p in patterns :
        if len(p) == 2 :
            one = p
        elif len(p) == 4 :
            four = p
        elif len(p) == 3 :
            seven = p
        elif len(p) == 7 :
            eight = p
    
    translation = {}

    counts = Counter(digits_string)
    for c, count in counts.items() :
        if count == 6 :
            translation[c] = 'B'
            B = c
        elif count == 4 :
            translation[c] = 'E'
            E = c
        elif count == 9 :
            translation[c] = 'F'
            F = c
        elif count == 7 :
            if c in four :
                translation[c] = 'D'
            else :
                translation[c] = 'G'
        elif count == 8 :
            if c in seven and c not in four :
                translation[c] = 'A'
            else :
                translation[c] = 'C'

    return translation
        
def signal2number(signal, translation) :
    segments = ''.join(sorted(translation[c] for c in signal))
    return segment2number[segments]

def output2number(output, translation) :
    digits = output.strip().split()
    value = 0
    for i, d in enumerate(digits) :
        value = 10 * value + signal2number(d, translation)
    return value

sum = 0
for case in cases :
    digits, output = case.split('|')
    translation = make_translation(digits)
    value = output2number(output, translation)
    sum += value

print(f"Part 2. Sum of output: {sum}")