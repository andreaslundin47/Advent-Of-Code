package day09

import java.io.File

val inputs: List<Pair<Char,Int>> = File("src/main/resources/day09.txt").readLines()
    .map { it.substringBefore(" ").single() to it.substringAfter(" ").toInt() }


class HeadTail(private var hx: Int = 0, private var hy: Int = 0) {
    var tx = hx
    var ty = hy

    private val tailVisits = hashSetOf( tx to ty )

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

    private fun isSeparated(dx: Int, dy: Int): Boolean {
        return maxOf (dx, dy) > 1
    }

    private fun tailFollow() {
        val dx = hx - tx
        val dy = hy - ty

        if ( isSeparated(dx, dy) ) {
            tx += dx.coerceIn(-1, 1)
            ty += dy.coerceIn(-1, 1)
        }

        tailVisits.add( tx to ty )
    }

    fun tailVisits(): Int = tailVisits.size
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