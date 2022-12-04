package day01

import java.io.File

val elfInventories = File("src/main/resources/day01.txt").readText().trim().split("\n\n")

fun main() {
    val caloriesPerElf: List<Int> = elfInventories.map { inventory ->
        inventory.lines().sumOf { it.toInt() }
    }

    val highestCaloriesInventory = caloriesPerElf.max()

    val sumOfThreeHighest = caloriesPerElf.sorted().takeLast(3).sum()

    println("Part 1. $highestCaloriesInventory")
    println("Part 2. $sumOfThreeHighest")
}