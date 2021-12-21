from itertools import cycle

with open('input', 'r') as f :
    p1_start = int(f.readline().strip().split()[-1])
    p2_start = int(f.readline().strip().split()[-1])


#### Part 1

class Dice :
    def __init__(self) :
        self.current = 0
        self.sides = 100
        self.uses = 0
    
    def roll(self) :
        self.current = (self.current % self.sides) + 1
        self.uses += 1
        return self.current
    
    def tripple_roll(self) :
        return sum(self.roll() for _ in range(3))
    

class Player:
    def __init__(self, position, die) :
        self.score = 0
        self.pos = position
        self.die = die
    
    def move(self) -> bool :
        steps = self.die.tripple_roll()
        self.pos = (self.pos-1+steps) % 10 + 1
        self.score += self.pos
        return self.score >= 1000

die = Dice()
players = [Player(p1_start, die), Player(p2_start, die)]

iterator = cycle(players)
for p in iterator :
    if p.move():
        break

loser = next(iterator)
print(f"Part 1. Answer: {loser.score * die.uses}")



#### 2

branching_numbers = {3:1, 4:3, 5:6, 6:7, 7:6, 8:3, 9:1}
initial_positions = (p1_start, p2_start)
initial_scores = (0,0)
WIN_LIMIT = 21

""" Return  [p1_wins, p2_wins] """
def game_branching(player_index, positions, scores) :
        wins = [0,0]
        for steps, number in branching_numbers.items() :
            new_pos = (positions[player_index]-1+steps) % 10 + 1
            pos, sco = list(positions), list(scores)
            pos[player_index] = new_pos
            sco[player_index] += new_pos
            if sco[player_index] >= WIN_LIMIT :
                wins[player_index] += number
            else :
                w1, w2 = game_branching((player_index+1)%2, pos, sco)
                wins[0] += number * w1
                wins[1] += number * w2
        return wins
 
wins = game_branching(0, initial_positions, initial_scores)

print(f"Part2. {wins}, Answer: {max(wins)}")

