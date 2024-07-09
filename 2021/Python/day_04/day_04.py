import copy

class Board :
    def __init__(self, numbers, index) :
        self.board_index = index
        self.nums = numbers
        self.filled = []
        self.filled_in_rows = [0] * 5
        self.filled_in_cols = [0] * 5
        self.won = False
    
    def mark(self, n) :
        if not self.won :
            self.filled.append(n)
            if n in self.nums :
                ind = self.nums.index(n)
                r, c = ind // 5, ind % 5
                self.filled_in_rows[r] += 1
                self.filled_in_cols[c] += 1
                if self.filled_in_rows[r] == 5 or self.filled_in_cols[c] == 5 :
                    self.won = True
    
    def is_winner(self) :
        return self.won

    def score(self) :
        return sum(v for v in self.nums if v not in self.filled) * self.filled[-1]


boards = []

with open('input', 'r') as f :
    bingo_numbers = [int(v) for v in f.readline().strip().split(',')]
    raw_boards = [board.strip() for board in f.read().split('\n\n')]
    for index, rb in enumerate(raw_boards) :
        board_numbers = []
        for row in rb.strip().split('\n') :
            for v in row.split() :
                board_numbers.append( int(v) )
        boards.append( Board(board_numbers, index) )


######## Part 1

def play_game_one(boards) :
    bs = copy.deepcopy(boards)
    for number in bingo_numbers :
        for board in bs :
            board.mark(number)
            if board.is_winner() :
                return board

score = play_game_one(boards).score()
print(f"Part 1. Score: {score}")


######## Part 2

def play_game_two(boards) :
    boards = copy.deepcopy(boards)
    last_winner = None

    for number in bingo_numbers :
        for board in boards:
            board.mark(number)
            if board.is_winner() :
                last_winner = board
        boards = [b for b in boards if not b.is_winner()]
    return last_winner

score = play_game_two(boards).score()
print(f"Part 2. Score: {score}")