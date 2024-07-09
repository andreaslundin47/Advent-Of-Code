
with open('input', 'r') as f :
    grid = [[c for c in line.strip()] for line in f.readlines()]


def turns_to_stabilize(grid) :
    height, width = len(grid), len(grid[0])
    turn = 0
    PRINT = False

    while True :

        if PRINT :
            print(f"Turn {turn}")
            print()
            s = ""
            for y in range(height) :
                for x in range(width) :
                    s += grid[y][x]
                s += '\n'
            print(s)

        turn += 1
        moves_in_turn = 0

        # Try to move '>'
        for y in range(0, height) :
            x = 0
            left, right = grid[y][0], grid[y][width-1]
            while x < width - 1 : 
                if grid[y][x] == '>'  and grid[y][x+1] == '.' :
                    grid[y][x] = '.'
                    grid[y][x+1] = '>'
                    moves_in_turn += 1
                    x += 2
                else :
                    x += 1

            # Handle wrap-around case
            if left == '.' and right == '>' :
                grid[y][0] = '>'
                grid[y][width-1] = '.'
                moves_in_turn += 1

        
        # Try to move 'v'
        for x in range(0, width) :
            y = 0
            top, bottom = grid[0][x], grid[height-1][x]
            while y < height - 1 :
                if grid[y][x] == 'v' and grid[y+1][x] == '.' :
                    grid[y][x] = '.'
                    grid[y+1][x] = 'v'
                    moves_in_turn += 1
                    y += 2
                else :
                    y += 1
            
            # Handle wrap-around case
            if bottom == 'v' and top == '.' :
                grid[0][x] = 'v'
                grid[height-1][x] = '.'
                moves_in_turn += 1
        

        if moves_in_turn == 0 :
            return turn 


print(f"Part 1. Pattern has stabilized on turn {turns_to_stabilize(grid)}")