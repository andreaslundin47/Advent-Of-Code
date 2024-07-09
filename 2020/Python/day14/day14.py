import re
from collections import deque

with open('input', 'r') as f:
    ops = [line.strip() for line in f.readlines()]

def parse_mask(text):
    numbers = text.split('=')[1].strip()
    enforce = ''.join('1' if c == '1' else '0' for c in numbers)
    block = ''.join('0' if c == '0' else '1' for c in numbers )
    return (int(enforce, 2), int(block, 2))

def parse_memset(text):
    addr, value = re.findall(r'\d+', text)
    return (int(addr), int(value))

def multi_mask_addr(address, mask):
    bin_addr = f"{address:b}".zfill(36)
    xed = ''.join([b if m == '0' else m for b,m in zip(bin_addr, mask)])
    book = deque([xed])
    while 'X' in book[0]:
        xaddress = book.popleft()
        book.append(xaddress.replace('X', '0', 1))
        book.append(xaddress.replace('X', '1', 1))
    assert 2 ** mask.count('X') == len(book)
    return [int(a, 2) for a in book]

# Part 1
memory = {}
force, block = 0, 0
for line in ops:
    if 'mask' in line:
        force, block = parse_mask(line)
    else:
        addr, value = parse_memset(line)
        value = (value | force) & block
        memory[addr] = value

s = sum(memory.values())
print(f"Part 1. Sum of values stored to memory: {s}")
        
# Part 2
memory = {}
for line in ops:
    if 'mask' in line:
        mask = line.split('=')[1].strip()
    else:
        addr, value = [int(v) for v in re.findall(r'\d+', line)]
        addresses = multi_mask_addr(addr, mask)
        for a in addresses: memory[a] = value

s = sum(memory.values())
print(f"Part 2. Sum of values stored to memory: {s}")