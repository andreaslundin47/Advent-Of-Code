from functools import reduce

with open('input', 'r') as f :
    hex_str = f.readline().strip()

bin_str = bin(int(hex_str, 16))[2:]
length = len(bin_str)
rem = len(bin_str) % 4
padding = (4 - rem) % 4
bin_str = bin_str.zfill(length + padding)


#### Part 1

""" Ignores the recursive structure, simply reads the 
    version ids from left to right 
"""
def version_sum(binstr) :
    bs = binstr
    version_sum = 0
    while '1' in bs :
        version_sum += int(bs[:3], 2)
        type = int(bs[3:6], 2)
        bs = bs[6:]
        if type == 4 :
            while bs[0] != '0' :
                bs = bs[5:]
            bs = bs[5:]
        else :
            if bs[0] == '0' :
                bs = bs[16:]
            else :
                bs = bs[12:]
    return version_sum

print(f"Part 1. Sum: {version_sum(bin_str)}")


#### Part 2

def perform_operation(opcode, stack) :
    if   opcode == 0 :
        return sum(stack)
    elif opcode == 1 :
        if len(stack) == 1:
            return stack[0]
        return reduce(lambda x,y: x*y, stack)
    elif opcode == 2 :
        return min(stack)
    elif opcode == 3 :
        return max(stack)
    elif opcode == 5 :
        if len(stack) != 2 :
            raise ValueError(f"Case GT: len is not 2, is {len(stack)}")
        return 1 if stack[0] > stack[1] else 0
    elif opcode == 6 :
        if len(stack) != 2 :
            raise ValueError(f"Case LT: len is not 2, is {len(stack)}")
        return 1 if stack[0] < stack[1] else 0
    elif opcode == 7 :
        if len(stack) != 2 :
            raise ValueError(f"Case EQ: len is not 2, is {len(stack)}")
        return 1 if stack[0] == stack[1] else 0


def parse_literal(binstr) :
    lit = ""
    prefix, group = binstr[0], binstr[1:5]
    while prefix != '0' :
        lit += group
        binstr = binstr[5:]
        prefix, group = binstr[0], binstr[1:5]
    lit += group
    binstr = binstr[5:]
    return int(lit, 2), binstr


def parse_bits_operator(binstr, op, nrbits) :
    subpacket_bits, remainder = binstr[:nrbits], binstr[nrbits:]
    literals = []
    while subpacket_bits :
        literal, subpacket_bits = parse(subpacket_bits)
        literals.append(literal)
    literal = perform_operation(op, literals)
    return literal, remainder


def parse_blocks_operator(binstr, op, nrblocks) :
    literals = []
    for block in range(nrblocks) :
        literal, binstr = parse(binstr)
        literals.append(literal)
    literal = perform_operation(op, literals)
    return literal, binstr


def parse(binstr) :
    type = int(binstr[3:6], 2)
    if type == 4 :
        literal, remainder = parse_literal(binstr[6:])
    else :
        if binstr[6] == '0':
            bits = int(binstr[7:22], 2)
            literal, remainder = parse_bits_operator(binstr[22:], op=type, nrbits=bits)
        else :
            blocks = int(binstr[7:18], 2)
            literal, remainder = parse_blocks_operator(binstr[18:], op=type, nrblocks=blocks)
    return literal, remainder


def compute(bitstr) :
    value, _ = parse(bitstr)
    return value


print(f"Part 2. Evaluated result: {compute(bin_str)}")