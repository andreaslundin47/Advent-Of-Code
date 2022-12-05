package day04

import java.io.File

data class Assignment(val lower: Int, val upper: Int) {

    companion object {
        fun fromString(range: String): Assignment {
            val (lower, upper) = range.split("-").map(String::toInt)
            return Assignment(lower, upper)
        }
    }

    operator fun contains(other: Assignment): Boolean = lower <= other.lower && upper > other.upper

    infix fun overlaps(other: Assignment): Boolean = upper >= other.lower && lower <= other.upper
}

val assignmentPairs: List<Pair<Assignment,Assignment>> = File("src/main/resources/day04.txt")
    .readLines().map { line ->
        line.split(",")
            .map { range -> Assignment.fromString(range) }
            .let { (a1, a2) -> Pair(a1, a2) }
    }

fun solvePartOne() {
    val overlaps: Int = assignmentPairs.count { (a1, a2) -> a1 in a2 || a2 in a1 }
    println("Part 1. Overlaps = $overlaps")
}

fun solvePartTwo() {
    val overlaps: Int = assignmentPairs.count { pair -> pair.first overlaps pair.second }
    println("Part 2. Overlaps = $overlaps")
}


fun main() {
    solvePartOne()
    solvePartTwo()
}
