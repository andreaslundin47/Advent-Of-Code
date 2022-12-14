package day14

import kotlin.math.abs

data class Vec2(val x: Int, val y: Int) {
    fun down() = Vec2(x,y+1)
    fun downLeft() = Vec2(x-1,y+1)
    fun downRight() = Vec2(x+1,y+1)

    operator fun plus(other: Vec2): Vec2 = Vec2(this.x + other.x, this.y + other.y)
    operator fun minus(other: Vec2): Vec2 = Vec2(this.x - other.x, this.y - other.y)
}

typealias Segment = Pair<Vec2,Vec2>
typealias Path = List<Segment>

val rockPaths: List<Path> = java.io.File("src/main/resources/day14.txt")
    .readLines()
    .map { line ->
        line
            .split(" -> ")
            .map { coordinate ->
                coordinate.split(",")
                    .let { (a,b) -> Vec2(a.toInt(), b.toInt()) }
            }
            .zipWithNext()
    }


fun Segment.toList(): List<Vec2> {
    val (start, end) = this
    val (dx, dy) = end - start

    val steps: Int  = maxOf( abs(dx), abs(dy) )
    val delta = Vec2( dx.coerceIn(-1,1), dy.coerceIn(-1,1) )

    return (0 until steps).scan(start) { acc, _ -> acc + delta }
}


fun List<Path>.toSet(): Set<Vec2> = this.flatMap { path -> path.flatMap { segment -> segment.toList() } }.toSet()


fun drawImage(rocks: Set<Vec2>, sands: Set<Vec2>) {
    val occupied = rocks + sands

    val minX = occupied.minOf { it.x }
    val minY = occupied.minOf { it.y }
    val maxX = occupied.maxOf { it.x }
    val maxY = occupied.maxOf { it.y }

    (minY..maxY).joinToString( separator = "\n") { y ->
        (minX..maxX).joinToString( separator = "" ) { x ->
            when (Vec2(x, y)) {
                in rocks -> "#"
                in sands -> "o"
                else -> "."
            }
        }
    }.let { println(it) }
}


fun solvePartOne() {
    val source = Vec2(500, 0)
    val rocks = rockPaths.toSet()

    val occupied = rocks.toMutableSet()
    val sandUnits = mutableSetOf<Vec2>()

    var sand = source
    val rockLimit: Int = rocks.maxOf { it.y }

    while (sand.y < rockLimit) {
        // Drop a new sand unit
        sand = source

        while (sand.y < rockLimit) {
            when {
                sand.down() !in occupied -> sand = sand.down()
                sand.downLeft() !in occupied -> sand = sand.downLeft()
                sand.downRight() !in occupied -> sand = sand.downRight()
                else -> {
                    sandUnits += sand
                    occupied += sand
                    break
                }
            }
        }
    }

    println("Part 1. ${sandUnits.size}")
}

fun solvePartTwo() {
    val source = Vec2(500, 0)
    val rocks = rockPaths.toSet()

    val floorLevel: Int = rocks.maxOf { it.y } + 2

    val occupied = rocks.toMutableSet()
    val sandUnits = mutableSetOf<Vec2>()

    while (source !in occupied) {
        // Drop a new sand unit
        var sand = source

        while (sand.y < floorLevel - 1) {
            when {
                sand.down() !in occupied -> sand = sand.down()
                sand.downLeft() !in occupied -> sand = sand.downLeft()
                sand.downRight() !in occupied -> sand = sand.downRight()
                else -> {
                    sandUnits += sand
                    occupied += sand
                    break
                }
            }
        }
        // Sand has reached the floor, and is at rest
        sandUnits += sand
        occupied += sand
    }

    println("Part 2. ${sandUnits.size}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}
