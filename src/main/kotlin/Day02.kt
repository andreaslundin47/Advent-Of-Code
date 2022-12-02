import java.io.File

val guide = File("src/main/resources/day02.txt")
    .readLines()
    .map { line ->
        val (a, b) = line.trim().split(" ")
        a.first() to b.first()
    }

sealed class RPSMove(val value: Int) {
    abstract fun play(opponent: RPSMove): Int
    abstract fun weakTo(): RPSMove
    abstract fun strongerThan(): RPSMove
}

class Rock: RPSMove(1)  {
    override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 3
            is Paper -> 0
            is Scissors -> 6
        }
    }

    override fun weakTo() = Paper()
    override fun strongerThan() = Scissors()
}

class Paper: RPSMove(2) {
     override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 6
            is Paper -> 3
            is Scissors -> 0
        }
    }

    override fun weakTo() = Scissors()
    override fun strongerThan() = Rock()
}

class Scissors: RPSMove(3) {
    override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 0
            is Paper -> 6
            is Scissors -> 3
        }
    }

    override fun weakTo() = Rock()
    override fun strongerThan() = Paper()
}

fun opponentMove(symbol: Char): RPSMove {
     return when(symbol) {
         'A' -> Rock()
         'B' -> Paper()
         'C' -> Scissors()
         else -> error("")
     }
}

fun firstReading(symbol: Char): RPSMove {
    return when(symbol) {
        'X' -> Rock()
        'Y' -> Paper()
        'Z' -> Scissors()
        else -> error("")
    }
}

fun secondReading(symbol: Char, opponent: RPSMove): RPSMove {
    return when(symbol) {
        'X' -> opponent.strongerThan()
        'Y' -> opponent
        'Z' -> opponent.weakTo()
        else -> error("")
    }
}

fun solvePart1() {
    val score = guide.sumOf { (their, mine) ->
        val opponentMove = opponentMove(their)
        val move = firstReading(mine)
        move.play(opponentMove)
    }
    println("Part 1. $score")
}

fun solvePart2() {
    var score = 0
    for ((their, mine) in guide) {
        val opponentMove = opponentMove(their)
        val move = secondReading(mine, opponentMove)
        score += move.play(opponentMove)
    }
    println("Part 2. $score")
}
fun main() {
    solvePart1()
    solvePart2()
}