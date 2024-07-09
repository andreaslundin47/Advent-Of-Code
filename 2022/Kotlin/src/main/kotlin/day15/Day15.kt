package day15

import kotlin.math.abs

val input = java.io.File("src/main/resources/day15.txt").readLines()

data class Vec2(val x: Int, val y: Int) {
    fun manhattanDist(other: Vec2) = abs(other.x-x) + abs(other.y - y)
}

data class Sensor(val pos: Vec2, val beacon: Vec2) {

    val dist = pos.manhattanDist(beacon)

    companion object {
        fun fromString(text: String): Sensor {
            val (sensorRaw, beaconRaw) = text.split(":")
            val (sx, sy) = sensorRaw.substringAfter("x=").split(", y=")
            val (bx, by) = beaconRaw.substringAfter("x=").split(", y=")
            val sensorPosition = Vec2(sx.toInt(), sy.toInt())
            val beaconPosition = Vec2(bx.toInt(), by.toInt())
            return Sensor(sensorPosition, beaconPosition)
        }
    }
}

fun List<IntRange>.merge(): List<IntRange> {
    val orderedRanges = this.sortedBy { it.first }.toMutableList()

    fun IntRange.extendWhileOverlapping(): IntRange {
        var startRange = this

        while (orderedRanges.isNotEmpty()) {
            val nextRange = orderedRanges.first()

            if (startRange.last + 1 >= nextRange.first) {
                startRange = startRange.first .. maxOf( startRange.last, nextRange.last)
                orderedRanges.removeFirst()
            } else {
                return startRange
            }
        }

        return startRange
    }

    val output = mutableListOf<IntRange>()

    while (orderedRanges.isNotEmpty()) {
        val startRange = orderedRanges.removeFirst()
        output += startRange.extendWhileOverlapping()
    }

    return output
}

fun solvePartOne() {
    val sensors = input.map { Sensor.fromString(it) }
    val specialY = 2_000_000

    val ranges = sensors
        .mapNotNull { sensor ->
            val dx = sensor.dist - abs(sensor.pos.y - specialY)
            if (dx >= 0) IntRange(sensor.pos.x - dx, sensor.pos.x + dx) else null
        }

    val coverageInRow = ranges.merge().sumOf {  it.last - it.first + 1 }
    val beaconsInRow = sensors.map { it.beacon }.filter { it.y == specialY }.toSet().size
    val numberEmpty = coverageInRow - beaconsInRow

    println("Part 1. $numberEmpty")
}


fun solvePartTwo() {
    val sensors = input.map { Sensor.fromString(it) }
    val upperLimit = 4_000_000

    // find first row with a split coverage range, this is our y!
    val y = (0 .. upperLimit).first { searchY ->
        val ranges = sensors
            .mapNotNull { sensor ->
                val dx = sensor.dist - abs(sensor.pos.y - searchY)
                if (dx >= 0) IntRange(sensor.pos.x - dx, sensor.pos.x + dx) else null
            }

            val mer = ranges.merge()
            mer.size > 1
        }

    // find first column with a split coverage range, this is our x!
    val x = (0 .. upperLimit).first { searchX ->
        val ranges = sensors
            .mapNotNull { sensor ->
                val dy = sensor.dist - abs(sensor.pos.x - searchX)
                if (dy >= 0) IntRange(sensor.pos.y - dy, sensor.pos.y + dy) else null
            }

            ranges.merge().size > 1
        }

    val frequency = 4_000_000L * x.toLong() + y.toLong()
    println("Part 2. x = $x, y = $y, tuning frequency = $frequency")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}