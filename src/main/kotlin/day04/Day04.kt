package day04

import java.io.File

data class Assignment(val lower: Int, val upper: Int) {

    companion object {
        fun fromString(range: String): Assignment {
            val (lower, upper) = range.split("-").map(String::toInt)
            return Assignment(lower, upper)
        }
    }

    infix fun hasFullOverlap(other: Assignment): Boolean {
        return this.lower >= other.lower && this.upper <= other.upper ||
                other.lower >= this.lower && other.upper <= this.upper
    }

    infix fun hasSomeOverlap(other: Assignment): Boolean {
        return !(this.upper < other.lower || this.lower > other.upper)
    }
}


val assignmentPairs: List<Pair<Assignment,Assignment>> = File("src/main/resources/day04.txt")
    .readLines()
    .map { line ->
        val (a1, a2) = line.split(",").map { range -> Assignment.fromString(range) }
        a1 to a2
    }

fun solvePartOne() {
    val overlaps = assignmentPairs.map { pair ->
        pair.first hasFullOverlap pair.second
    }.count { it }

    println("Part 1. Overlaps = $overlaps")
}

fun solvePartTwo() {
    val overlaps = assignmentPairs.map { pair ->
        pair.first hasSomeOverlap pair.second
    }.count { it }

    println("Part 2. Overlaps = $overlaps")

}


fun main() {
    solvePartOne()
    solvePartTwo()
}
