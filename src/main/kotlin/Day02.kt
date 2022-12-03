import java.io.File

val guide = File("src/main/resources/day02.txt")
    .readLines()
    .map { line ->
        val (a, b) = line.trim().split(" ")
        a.first() to b.first()
    }

sealed class RPSMove(val value: Int, val weakTo: RPSMove?, val strongerThan: RPSMove?) {
    abstract fun play(opponent: RPSMove): Int
}

class Rock: RPSMove(1, Paper(), Scissors())  {
    override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 3
            is Paper -> 0
            is Scissors -> 6
        }
    }
}

class Paper: RPSMove(2, Scissors(), Rock()) {
     override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 6
            is Paper -> 3
            is Scissors -> 0
        }
    }
}

class Scissors: RPSMove(3, Rock(), Paper()) {
    override fun play(opponent: RPSMove): Int {
        return value + when (opponent) {
            is Rock -> 0
            is Paper -> 6
            is Scissors -> 3
        }
    }
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
        'X' -> opponent.strongerThan ?: opponent
        'Y' -> opponent
        'Z' -> opponent.weakTo ?: opponent
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