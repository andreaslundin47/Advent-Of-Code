package day05

import java.io.File

data class Move(val count: Int, val from: Int, val to: Int)

fun String.toMove(): Move =
    split(" ").mapNotNull(String::toIntOrNull).let { d -> Move(d[0], d[1] -1, d[2] - 1) }

class CrateStacks(private val stacks: MutableList<List<Char>>) {
    companion object {
        fun fromString(input: String): CrateStacks {

            val stackLabelsBar = input.lines().last()
            val rawStacks = input.lines().dropLast(1).reversed()

            val stackIDs = stackLabelsBar.filter { it.isDigit() }

            val stacks = stackIDs.map { id ->
                val columnIndex = stackLabelsBar.indexOf(id)
                rawStacks.map { row -> row[columnIndex] }.filter { it.isLetter() }
            }

            return CrateStacks(stacks.toMutableList())
        }
    }

    fun makeSingleMoves(move: Move) {
        stacks[move.to] += stacks[move.from].takeLast(move.count).reversed()
        stacks[move.from] = stacks[move.from].dropLast(move.count)
    }

    fun makeBulkMove(move: Move) {
        stacks[move.to] += stacks[move.from].takeLast(move.count)
        stacks[move.from] = stacks[move.from].dropLast(move.count)
    }

    fun readTop(): String = stacks.map { stack -> stack.last() }.joinToString(separator = "")
}

fun solvePartOne() {
    val crateStacks = CrateStacks.fromString(stacksInput)
    moves.forEach { move -> crateStacks.makeSingleMoves(move) }
    println("Part 1. ${crateStacks.readTop()}")
}

fun solvePartTwo() {
    val crateStacks = CrateStacks.fromString(stacksInput)
    moves.forEach { move -> crateStacks.makeBulkMove(move) }
    println("Part 1. ${crateStacks.readTop()}")
}


val rawInput = File("src/main/resources/day05.txt").readText().trim()
val stacksInput = rawInput.substringBefore("\n\n")
val movesInput = rawInput.substringAfter("\n\n")

val moves: List<Move> = movesInput.lines().map { it.toMove() }

fun main() {
    solvePartOne()
    solvePartTwo()
}