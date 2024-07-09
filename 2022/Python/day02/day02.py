with open('day02.txt', 'r') as f :
    guide = []
    for line in f.readlines() :
        a, b = line.strip().split()
        opponentMove = a.replace('A', 'R').replace('B', 'P').replace('C', 'S')
        guide.append( (opponentMove, b) )


def singleTurnScore(move, opposingMove) :
    # Rock     -> 1
    # Paper    -> 2
    # Scissors -> 3
    myMoveScore = {'R': 1, 'P': 2, 'S': 3}

    myMove = myMoveScore[move]
    theirMove = myMoveScore[opposingMove]

    delta = myMove - theirMove

    if delta == 0 :
        return myMove + 3

    elif delta == 1 :
        return myMove + 6
    elif delta == -1 :
        return myMove
    elif delta == 2 :
        return myMove
    else :
        return myMove + 6


def firstReading(rule) :
    translation = {'X': 'R', 'Y': 'P', 'Z': 'S'}
    return translation[rule]


def secondReading(rule, opponent) :
    moveThatLosesTo = {'R': 'S', 'P': 'R', 'S': 'P'}
    moveThatWinsOver = {'R': 'P', 'P': 'S', 'S': 'R'}

    if   rule == 'X' :
        return moveThatLosesTo[opponent]
    elif rule == 'Z' :
        return moveThatWinsOver[opponent]
    else :
        return opponent



def solvePartOne() :
    score = 0
    for opponentMove, rule in guide :
        move = firstReading(rule)
        score += singleTurnScore(move, opponentMove)
    print(f"Part 1. {score}")


def solvePartTwo() :
    score = 0
    for opponentMove, rule in guide :
        move = secondReading(rule, opponentMove)
        score += singleTurnScore(move, opponentMove)
    print(f"Part 2. {score}")


solvePartOne()
solvePartTwo()