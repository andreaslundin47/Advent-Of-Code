
var rawInput = java.io.File("src/main/resources/input-01.txt").readText().trim()

class Day01(input: String) {

    private val digitWords = listOf("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
    private val lines = input.lines()

    private fun String.toValueString(): String {
        return (digitWords.indexOf(this) + 1).toString()
    }

    private fun String.substituteWords(): String =
        mapIndexed { i, c ->
            when (c.isDigit()) {
                true -> c
                false -> {
                    val wordMatch = digitWords.firstOrNull { substring(i).startsWith(it) }
                    wordMatch?.toValueString() ?: ""
                }
            }
        }
        .joinToString()


    private fun String.calibrationValue(): Int =
        "${first{ it.isDigit() }}${last { it.isDigit() }}".toInt()

    fun solvePartOne() {
        val sum = lines.sumOf { line -> line.calibrationValue() }
        println("Part 1. Sum = $sum")
    }

    fun solvePartTwo() {
        val sum = lines.sumOf { line -> line.substituteWords().calibrationValue() }
        println("Part 2. Sum = $sum")
    }
}

fun main() {
    val d = Day01(rawInput)
    d.solvePartOne()
    d.solvePartTwo()
}