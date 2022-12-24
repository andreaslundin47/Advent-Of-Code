package day23

val input = java.io.File("src/main/resources/day23.txt").readText().trim()

class Rectangle(val xRange: IntRange, val yRange: IntRange) {
    fun area() = xRange.count() * yRange.count()
}

data class Pos(val x: Int, val y: Int) {
    fun north() = Pos(x, y-1)
    fun south() = Pos(x, y+1)
    fun west() = Pos(x-1, y)
    fun east() = Pos(x+1, y)

    fun northern() = listOf(-1, 0, 1).map { Pos(x + it, y - 1) }
    fun southern() = listOf(-1, 0, 1).map { Pos(x + it, y + 1) }
    fun western() = listOf(-1, 0, 1).map { Pos(x - 1, y + it) }
    fun eastern() = listOf(-1, 0, 1).map { Pos(x + 1, y + it) }
}

class ElfTracker(private val elfPositions: Set<Pos>, val turn: Int = 0) {

    companion object {
        fun fromString(text: String): ElfTracker {
            val elves = text.lines().flatMapIndexed { y, line ->
                line.mapIndexedNotNull { x, entry ->
                    if (entry == '#') Pos(x, y) else null
                }
            }.toSet()
            return ElfTracker(elves, turn = 0)
        }
    }

    infix fun samePositions(other: ElfTracker) = elfPositions == other.elfPositions

    private fun smallestBoundingRectangle(): Rectangle {
        val xRange = elfPositions.minOf { it.x } .. elfPositions.maxOf { it.x }
        val yRange = elfPositions.minOf { it.y } .. elfPositions.maxOf { it.y }
        return Rectangle(xRange, yRange)
    }

    fun countOpenGround() = smallestBoundingRectangle().area() - elfPositions.size

    fun draw() {
        val rec = smallestBoundingRectangle()
        println()
        for (y in rec.yRange) {
            for (x in rec.xRange) {
                when ( Pos(x,y) ) {
                    in elfPositions -> print("#")
                    else -> print(".")
                }
            }
            println()
        }
        println()
    }

    private fun destinationOfElf(pos: Pos): Pos {
        val neighbours = (pos.northern() + pos.southern() + pos.western() + pos.eastern()).toSet()

        // Don't move if no other elf is within the eight adjacent positions
        if (neighbours.all { it !in elfPositions }) {
            return pos
        }

        // Otherwise try directions in the given order, and pick the first that is viable
        val startIndex = turn % 4

        for (i in 0..3) {
            when ( (startIndex + i) % 4 ) {
                0 -> if (!pos.northern().any { it in elfPositions }) return pos.north()
                1 -> if (!pos.southern().any { it in elfPositions }) return pos.south()
                2 -> if (!pos.western().any { it in elfPositions }) return pos.west()
                3 -> if (!pos.eastern().any { it in elfPositions }) return pos.east()
                else -> error("Bad index!")
            }
        }

        // Stay if no direction was viable
        return pos
    }

    fun next(): ElfTracker {
        // Determine where each elf would like to go
        val elvesWithTargets = elfPositions.map { it to destinationOfElf(it) }

        // Find all locations where more than one elf want to go
        val targets = mutableSetOf<Pos>()
        val forbidden = mutableSetOf<Pos>()

        elvesWithTargets.forEach { (_, target) ->
            if (target in targets) {
                forbidden += target
            }
            targets += target
        }

        // Update all positions and return the new state.
        // Only elves with unique destinations will move
        val nextPositions = elvesWithTargets.map { (elf, target) ->
            if (target !in forbidden) target else elf
        }.toSet()

        return ElfTracker(nextPositions, turn = turn + 1)
    }
    
}


fun solvePartOne() {

    var elves = ElfTracker.fromString(input)
    repeat(10) { elves = elves.next() }
    println("Part 1. Ground = ${elves.countOpenGround()}")
}

fun solvePartTwo() {
    var elfTracker = ElfTracker.fromString(input)
    while (true) {
        val updatedElfTracker = elfTracker.next()
        if (elfTracker samePositions updatedElfTracker) break
        elfTracker = updatedElfTracker
    }
    println("Part 2. No change in round ${elfTracker.turn + 1}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}