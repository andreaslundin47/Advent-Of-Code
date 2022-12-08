package day08

import java.io.File
import kotlin.math.max

typealias Grid = List<List<Int>>

//  Part 1.  Parse input  --------------------------------------------------------------------------------------

val heightMap: Grid = File("src/main/resources/day08.txt")
    .readLines()
    .map { row -> row.map(Char::digitToInt) }

//  Part 1.  ---------------------------------------------------------------------------------------------------

fun <T> List<List<T>>.transpose(): List<List<T>> =
    first().indices.map { columnIndex -> this.map { it[columnIndex] } }


fun List<Int>.visibilityAlongLine(): List<Boolean> {

    fun List<Int>.visibilityFromLeft(): List<Boolean> =
    drop(1).scan(first() to true ) { acc, treeHeight ->
        val highestSoFar = acc.first
        max(highestSoFar, treeHeight) to (treeHeight > highestSoFar)
    }.map { it.second }

    val visibleFromLeft = visibilityFromLeft()
    val visibleFromRight = reversed().visibilityFromLeft().reversed()
    return visibleFromLeft.zip(visibleFromRight) { a,b -> a || b }
}


fun solvePartOne() {
    val visibilityHorizontal = heightMap.map { it.visibilityAlongLine() }
    val visibilityVertical = heightMap.transpose().map { it.visibilityAlongLine() }.transpose()

    val visibility = visibilityHorizontal.zip(visibilityVertical) { l1, l2 -> l1.zip(l2) { t1, t2 -> t1 || t2 } }
    val countVisible = visibility.flatten().count { it }

    println("Part 1. Visible = $countVisible")
}

//  Part 2.  ---------------------------------------------------------------------------------------------------

fun List<Int>.countSeenByFirst(): Int {
    val shorterVisible = drop(1).takeWhile { it < first() }.size
    return shorterVisible + if (shorterVisible < size - 1) 1 else 0
}

fun Grid.scenicScore(x: Int, y: Int): Int {
    val row = this[y]
    val col = this.map { it[x] }

    val left = row.take(x+1).reversed().countSeenByFirst()
    val right = row.drop(x).countSeenByFirst()
    val up = col.take(y+1).reversed().countSeenByFirst()
    val down = col.drop(y).countSeenByFirst()

    return left * right * up * down
}

fun Grid.scenicMap(): Grid = this.mapIndexed { j, row -> row.indices.map { i -> this.scenicScore(i, j) } }

fun solvePartTwo() {
    val highScore = heightMap.scenicMap().flatten().max()
    println("Part 2. Highest scenic score = $highScore")
}

//  Main  ------------------------------------------------------------------------------------------------------

fun main() {
    solvePartOne()
    solvePartTwo()
}