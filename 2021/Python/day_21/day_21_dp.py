from functools import lru_cache

with open('input', 'r') as f :
    p1_start = int(f.readline().strip().split()[-1])
    p2_start = int(f.readline().strip().split()[-1])

roll_ratios = {3:1, 4:3, 5:6, 6:7, 7:6, 8:3, 9:1}
WIN_LIMIT = 21

game_cache = {}

@lru_cache(maxsize=None)
def count_wins(pos_A, pos_B, score_A, score_B) :
    """ Assume that it's player A's turn to move when the function
        is called. 
    """
    wins_A, wins_B = 0, 0
    for steps, counts in roll_ratios.items() :
        pos_A_new = (pos_A-1+steps) % 10 + 1
        score_A_new = score_A + pos_A_new

        if score_A_new >= WIN_LIMIT :
            wins_A += counts
        else :
            wB, wA = count_wins(pos_B, pos_A_new, score_B, score_A_new)
            wins_A += wA * counts
            wins_B += wB * counts

    return wins_A, wins_B
        
print(f"Part 2. Wins: {max(count_wins(p1_start, p2_start, 0, 0))}")