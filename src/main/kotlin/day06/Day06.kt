package day06

import java.io.File

val input: String = File("src/main/resources/day06.txt").readLines().first().trim()


fun solvePartOne() {
    val leadingString = input
        .scan("") { acc, char -> acc + char }
        .firstOrNull { it.takeLast(14).toSet().size == 14 } ?: ""
    println("Part 1. ${leadingString.length}")
}

fun solvePartTwo() {
    val leadingString = input
        .scan("") { acc, char -> acc + char }
        .firstOrNull { it.takeLast(4).toSet().size == 4 } ?: ""
    println("Part 2. ${leadingString.length}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}