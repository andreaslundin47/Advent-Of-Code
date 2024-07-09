import re

with open('input', 'r') as f:
    pattern = re.compile(r"([\(\)*+(\d+)])")
    tasks = [list(pattern.findall(line.strip())) for line in f.readlines()]
    ex1 = pattern.findall("1 + (2 * 3) + (4 * (5 + 6))")
    ex2 = pattern.findall("2 * 3 + (4 * 5)")
    ex3 = pattern.findall("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    ex4 = pattern.findall("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    ex5 = pattern.findall("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")

class ParseStack:
    def __init__(self):
        self.st = []

    def push_number(self, number):
        if self.st == []:
            self.st.append(number)
        elif self.st[-1] == '+':
            self.st.pop(); term = self.st.pop()
            result = str(int(term) + int(number))
            self.push_number(result)
        elif self.st[-1] == '*':
            self.st.pop(); term = self.st.pop()
            result = str(int(term) * int(number))
            self.push_number(result)
        else:
            self.st.append(number)

    def push_other(self, token):
        if token == ')':
            between = self.st.pop()
            self.st.pop()
            self.push_number(between)
        else:
            self.st.append(token)

    def push(self, token):
        if token.isnumeric():
            self.push_number(token) 
        else:
            self.push_other(token)

    def parse(self, syms):
        for token in syms:
            self.push(token)

    def result(self):
        return int(self.st.pop())

def prob2num(tokens):
    stack = ParseStack()
    stack.parse(tokens)
    return stack.result()

# Part 1
s = sum(prob2num(toks) for toks in tasks)
print(f"Part 1. Sum of results: {s}")



class SecondParseStack:
    def __init__(self):
        self.st = []

    def push_number(self, number):
        if self.st == []:
            self.st.append(number)
        elif self.st[-1] == '+':
            self.st.pop(); term = self.st.pop()
            result = str(int(term) + int(number))
            self.push_number(result)
        elif self.st[-1] == '*':
            self.st.append(number)
        else:
            self.st.append(number)

    def push_other(self, token):
        if token == ')':
            self.reduce_stack()
        else:
            self.st.append(token)

    def reduce_stack(self):
        value = self.st.pop()
        while self.st and self.st[-1] != '(':
            if self.st[-1] == '*':
                self.st.pop(); fac = self.st.pop()
                value = str(int(fac) * int(value))
        if self.st and self.st[-1] == '(':
            self.st.pop()
        self.push_number(value)

    def push(self, token):
        if token.isnumeric():
            self.push_number(token) 
        else:
            self.push_other(token)

    def parse(self, syms):
        for token in syms:
            self.push(token)

    def result(self):
        self.reduce_stack()
        return int(self.st.pop())

def secondProb2num(tokens):
    stack = SecondParseStack()
    stack.parse(tokens)
    return stack.result()

# Part 2
assert secondProb2num(ex1) == 51
assert secondProb2num(ex2) == 46
assert secondProb2num(ex3) == 1445
assert secondProb2num(ex4) == 669060
assert secondProb2num(ex5) == 23340

s = sum(secondProb2num(toks) for toks in tasks)
print(f"Part 2. Sum of results: {s}")