package day05

import java.io.File

val stacksInput = File("src/main/resources/day05.txt").readText().substringBefore("\n\n").trim()
val movesInput = File("src/main/resources/day05.txt").readText().substringAfter("\n\n").trim()

data class Move(val steps: Int, val from: Int, val to: Int)

fun String.toMove(): Move {
    val parts = this.trim().split(" ")
    return Move(parts[1].toInt(), parts[3].toInt()-1, parts[5].toInt()-1)
}

class CrateStacks(private val stacks: List<MutableList<Char>>) {
    companion object {
        fun fromString(input: String): CrateStacks {
            val rawStackIds = input.split("\n").reversed().first()
            val rawStacks = input.split("\n").reversed().drop(1)

            val ids = rawStackIds.filter { it != ' ' }
            val padding = rawStackIds.length

            val stacks = mutableListOf<MutableList<Char>>()

            for (id in ids) {
                val column = rawStackIds.indexOf(id)
                val stack: List<Char> = rawStacks.map { line -> line.padEnd(padding)[column] }.filter { it != ' ' }
                stacks.add(stack.toMutableList())
            }

            return CrateStacks(stacks.toList())
        }
    }

    fun makeMove(move: Move) {
        stacks[move.to] +=  stacks[move.from].takeLast(move.steps).reversed()
        for (i in 1..move.steps) {
            stacks[move.from].removeLast()
        }
    }

    fun makeMultiMove(move: Move) {
        stacks[move.to] +=  stacks[move.from].takeLast(move.steps)
        for (i in 1..move.steps) {
            stacks[move.from].removeLast()
        }
    }

    fun readTop(): String = stacks.map { stack -> stack.last() }.joinToString(separator = "")
}


fun solvePartOne() {
    val stacks = CrateStacks.fromString(stacksInput)
    val moves = movesInput.split("\n").map { it.toMove() }
    moves.forEach { move -> stacks.makeMove(move) }

    println("Part 1. ${stacks.readTop()}")
}

fun solvePartTwo() {
    val stacks = CrateStacks.fromString(stacksInput)
    val moves = movesInput.split("\n").map { it.toMove() }
    moves.forEach { move -> stacks.makeMultiMove(move) }

    println("Part 1. ${stacks.readTop()}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}