
with open('input', 'r') as f:
    instructions = [line.strip().split() for line in f.readlines()]
    program = [(op, int(arg)) for op, arg in instructions]

def run(program):
    seen = set()
    ip = 0
    acc = 0
    while ip < len(program) and ip not in seen:
        seen.add(ip)

        op, arg = program[ip]
        if op == 'nop':
            ip += 1
        elif op == 'acc':
            acc += arg
            ip += 1
        elif op == 'jmp':
            ip += arg
        else:
            raise ValueError(f'Bad Op Code: {ip}, {acc}, {op}')

    return (ip not in seen, acc)

# Part 1
_, acc = run(program)
print(f"Part 1. Acc = {acc}")

# Part 2
for ip, (op, arg) in enumerate(program):
    if op == 'acc':
        continue
    candidate = program.copy()
    instr = {'nop':'jmp', 'jmp':'nop'}[op]
    candidate[ip] = (instr, arg)

    terminated, acc = run(candidate)
    if terminated:
        print(f"Part 2. After termination, Acc = {acc}")
        break