
with open('input', 'r') as f:
    numbers = [int(row.strip()) for row in f.readlines()]

def verify(value, preceeding):
    for i, a in enumerate(preceeding):
        for b in preceeding[i:]:
            if a + b == value:
                return True
    return False

def contiguous_set(value, numbers):
    for set_len in range(2, len(numbers)):
        acc = sum(numbers[0:set_len])
        ri = set_len-1
        for shift in range(len(numbers)-set_len):
            if acc == value:
                return (shift, ri+shift)
            if (ri+shift+1 < len(numbers)):
                acc = acc - numbers[shift] + numbers[ri+shift+1]
    return (-1, -1)


# Part 1
span = 25
for i,v in enumerate(numbers):
    if i < span:
        continue
    
    if verify(v, numbers[i-span:i]) == False:
        target_value, target_idx = v, i
        print(f"Part 1. First invalid number: {target_value}")

# Part 2
lo, hi = contiguous_set(target_value, numbers)
smallest = min(numbers[lo:hi+1])
largest = max(numbers[lo:hi+1])
s = smallest + largest

print(f"Part 2. n1={smallest}, n2={largest}, s={s}")
