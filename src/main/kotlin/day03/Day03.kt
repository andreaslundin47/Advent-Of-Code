package day03

import java.io.File

val rucksacks: List<String> = File("src/main/resources/day03.txt").readLines()

fun Char.itemPriority(): Int = if (this.isUpperCase()) (this - 'A' + 27) else (this - 'a' + 1)

fun solvePartOne() {
    val prioritySum: Int = rucksacks.sumOf { rucksack ->
        val compSize = rucksack.length / 2
        val firstCompartment = rucksack.take(compSize).toSet()
        val secondCompartment = rucksack.takeLast(compSize).toSet()
        val sharedItem: Char = firstCompartment.intersect(secondCompartment).first()

        sharedItem.itemPriority()
    }
    println("Part 1. Sum = $prioritySum")
}

fun solvePartTwo() {
    val prioritySum: Int = rucksacks.chunked(3).sumOf { elfGroup ->
        val rucksackSets = elfGroup.map { rucksack -> rucksack.toSet() }
        val sharedItem: Char = rucksackSets.reduce { acc, sack -> acc.intersect(sack) }.first()

        sharedItem.itemPriority()
    }

    println("Part 2. Sum = $prioritySum")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}