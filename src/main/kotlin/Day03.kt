import java.io.File

val rucksacks: List<String> = File("src/main/resources/day03.txt").readLines()

fun Char.itemPriority(): Int = if (this.isUpperCase()) (this - 'A' + 27) else (this - 'a' + 1)

fun solvePartOne() {
    val prioritySum = rucksacks.sumOf { rucksack ->
        val compSize = rucksack.length / 2
        val firstCompartment = rucksack.take(compSize).toCharArray().toSet()
        val secondCompartment = rucksack.takeLast(compSize).toCharArray().toSet()
        val intersection = firstCompartment.intersect(secondCompartment)

        intersection.first().itemPriority()
    }
    println("Part 1. Sum = $prioritySum")
}

fun solvePartTwo() {
    val elfGroups = rucksacks.chunked(3)
    val prioritySum = elfGroups.sumOf { group ->
        val rucksackSets = group.map { rucksack -> rucksack.toCharArray().toSet() }
        val sharedItem = rucksackSets.reduce { acc, sack -> acc.intersect(sack) }.first()

        sharedItem.itemPriority()
    }

    println("Part 2. Sum = $prioritySum")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}