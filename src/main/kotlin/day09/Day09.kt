package day09

import java.io.File
import kotlin.math.abs

val inputs: List<Pair<Char,Int>> = File("src/main/resources/day09.txt").readLines()
    .map { it.substringBefore(" ").single() to it.substringAfter(" ").toInt() }


class HeadTail(private var hx: Int = 0, private var hy: Int = 0) {
    var tx = hx
    var ty = hy

    private val seenByTail = hashSetOf( tx to ty )

    fun move(direction: Char) {
        when (direction) {
            'U' -> hy += 1
            'D' -> hy -= 1
            'L' -> hx -= 1
            'R' -> hx += 1
            else -> error("Odd direction")
        }
        tailFollow()
    }

    fun move(x: Int, y: Int) {
        hx = x
        hy = y
        tailFollow()
    }

    private fun tailFollow() {
        val dx = hx - tx
        val dy = hy - ty

        if (abs(dx) + abs(dy) > 2) {
            tx += dx / abs(dx)
            ty += dy / abs(dy)
        }
        else if (abs(dx) > 1) {
            tx += dx / abs(dx)
        }
        else if (abs(dy) > 1) {
            ty += dy / abs(dy)
        }

        seenByTail.add( tx to ty )
    }

    fun tailVisits(): Int = seenByTail.count()
}

class Rope(length: Int) {
    private val knots: List<HeadTail> = Array(length-1) { HeadTail() }.toList()

    fun move(direction: Char) {
        knots.first().move(direction)

        knots.zipWithNext { k1, k2 ->
            k2.move( k1.tx, k1.ty )
        }
    }

    fun tailVisits(): Int = knots.last().tailVisits()
}

fun solvePartOne() {
    val ht = HeadTail()
    for ( (direction, times) in inputs ) {
        repeat(times) { ht.move(direction) }
    }
    println("Part 1. ${ht.tailVisits()}")
}

fun solvePartTwo() {
    val rope = Rope(10)
    for ( (direction, times) in inputs ) {
        repeat(times) { rope.move(direction) }
    }
    println("Part 2. ${rope.tailVisits()}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}