
# Instance
data = [3, 6, 4, 2, 9, 7, 5, 8, 1]

class Game:
    def __init__(self, numbers):
        self.current = numbers[0]
        self.lo = min(numbers)
        self.hi = max(numbers)
        self.cups = {}

        for c1, c2 in zip(numbers[:-1], numbers[1:]):
            self.cups[c1] = c2
        self.cups[numbers[-1]] = numbers[0]

    def move(self):
        # find three cups
        tail = []
        at = self.current
        for i in range(3):
            at = self.cups[at]
            tail.append(at)
        # Determine the destination
        destination = self.current - 1
        while destination in tail or destination < self.lo:
            destination -= 1
            if destination < self.lo:
                destination = self.hi
        # Reconnect
            # Remove tail
        tail_connected_to = self.cups[tail[-1]]
        self.cups[self.current] = tail_connected_to
            # Connect end of tail
        to_follow_trail = self.cups[destination] 
        self.cups[tail[-1]] = to_follow_trail
            # Connect destination to start of tail
        self.cups[destination] = tail[0]
            # Set new current
        self.current = self.cups[self.current]

    def array(self, start=None):
        start = self.current if start == None else start
        curr = start
        s = [start]
        while self.cups[curr] != start:
            curr = self.cups[curr]
            s.append(curr)
        return s

# Part 1
c = Game(data)
for i in range(100):
    c.move()
seq = ''.join(str(a) for a in c.array(start=1))
print(f"Part 1. Labels after one: {seq[1:]}")

# Part 2
d = data + list(range(max(data)+1, 1_000_001))
c = Game(d)
for i in range(10_000_000):
    c.move()

after_1 = c.cups[1]
after_after_1 = c.cups[after_1]
prod = after_1 * after_after_1
print(f"Part 2. Product: {prod}")