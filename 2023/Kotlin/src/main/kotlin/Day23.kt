import java.util.PriorityQueue
import kotlin.math.absoluteValue
import kotlin.math.max

fun main() {
    val input = java.io.File("src/main/resources/input-23.txt").readText().trim()
    //val input = java.io.File("src/main/resources/input-23-sample1.txt").readText().trim()
    val day = Day23(input)
    //day.solvePartOne()
    day.solvePartTwo()
}

class Day23(input: String) {

    private val north = Point(0, -1)
    private val south = Point(0, 1)
    private val west = Point(-1, 0)
    private val east = Point(1, 0)

    data class Point(val x: Int, val y: Int) {
        operator fun plus(other: Point): Point {
            return Point(x + other.x, y + other.y)
        }

        operator fun minus(other: Point): Point {
            return Point(x - other.x, y - other.y)
        }

        fun isAdjacent(other: Point): Boolean {
            return (x - other.x).absoluteValue + (y - other.y).absoluteValue == 1
        }
    }


    private val points: Set<Point> = input.lines().flatMapIndexed { y, line ->
        line.mapIndexedNotNull { x, c ->
            if (c != '#') {
                Point(x, y)
            } else {
                null
            }
        }
    }
        .toSet()

    private val symbols: Map<Point, Char> = input.lines().flatMapIndexed { y, line ->
        line.mapIndexedNotNull { x, c ->
            if (c in "<>^v") {
                Point(x, y) to c
            } else {
                null
            }
        }
    }
    .toMap()

    fun Point.neighbours(): List<Point> {
        return setOf(east, west, north, south)
            .map { this + it }
            .filter { it in points }
    }

    fun Point.IsExit(other: Point, directed: Boolean): Boolean {
        if (!isAdjacent(other) || other !in points) {
            return false
        }

        if (!directed) {
            return true
        }

        val valid = listOf(north to '^', south to 'v', west to '<', east to '>')
        val sym = symbols.getValue(other)
        val shift = other - this

        return (shift to sym) in valid
    }


    private val maxY: Int = points.maxOf { it.y }
    private val start: Point = points.first { it.y == 0 }
    private val finish: Point = points.first { it.y == maxY }

    private val intersections: List<Point> = points.filter { p -> p.neighbours().size > 2 }

    private val endPoints = intersections + finish

    private val entryPoints: Map<Point, List<Point>> = (
        intersections
            .associateWith { entry -> entry.neighbours().filter { n -> entry.IsExit(n, true) } }
            .toMutableMap() +
                (start to listOf(start + south))
            ).toMap()

    private val entryPoints2: Map<Point, List<Point>> = (
            intersections
                .associateWith { entry -> entry.neighbours().filter { n -> entry.IsExit(n, false) } }
                .toMutableMap() +
                    (start to listOf(start + south))
            ).toMap()

    private val edges: Map<Point, Set<Edge>> = entryPoints
        .map { (entry, exits) ->
            entry to exits.map { ex -> entry.findEdge(ex) }.toSet()
        }.toMap()

    private val edges2: Map<Point, Set<Edge>> = entryPoints2
        .map { (entry, exits) ->
            entry to exits.map { ex -> entry.findEdge2(ex) }.toSet()
        }.toMap()

    data class Edge(val destination: Point, val length: Int)

    private fun Point.findEdge(exit: Point): Edge {
        var dist = 1
        var current = this
        var next = exit

        while (next !in endPoints) {
            val nextNext = next.neighbours().filterNot { it == current }.first()
            current = next
            next = nextNext
            dist += 1
        }

        return Edge(next, dist)
    }

    private fun Point.findEdge2(exit: Point): Edge {
        var dist = 1
        var current = this
        var next = exit

        val endP = endPoints + start

        while (next !in endP) {
            val nextNext = next.neighbours().filterNot { it == current }.first()
            current = next
            next = nextNext
            dist += 1
        }

        return Edge(next, dist)
    }

     fun solvePartOne() {
         data class SearchState(val vertex: Point, val distance: Int)

         val distances = mutableMapOf<Point, Int>(start to 0)

         val queue = PriorityQueue<SearchState>(compareBy { it.distance })
         queue.add(SearchState(start, 0))

         while (queue.isNotEmpty()) {
             val (current, steps) = queue.remove()

             val edges = edges[current] ?: continue

             for (edge in edges) {
                 val (neighbour, moreSteps) = edge
                 if (neighbour !in distances) {
                     distances[neighbour] = steps - moreSteps
                     queue.add(SearchState(neighbour, steps - moreSteps))
                 }
                 else if(steps - moreSteps < distances.getValue(neighbour)) {
                     distances[neighbour] = steps - moreSteps
                     queue.add(SearchState(neighbour, steps - moreSteps))
                 }
             }
         }

         val longest = -distances.getValue(finish)
         println("Part 1. Longest distance = $longest")
    }

    fun solvePartTwo() {
        val remaining = intersections.toSet() + finish
        val (_, maxSteps) = steps(start, remaining)
        println("Part 2. steps = $maxSteps")
    }

    private fun steps(current: Point, remaining: Set<Point>): Pair<Boolean, Int> {
        if (current == finish) {
            return true to 0
        }

        val edges = edges2[current] ?: return false to 0

        var best = 0
        var worked = false
        for ((next, steps) in edges) {
            if (next in remaining) {
                val (success, count) = steps(next, remaining - next)
                if (success) {
                    worked = true
                    best = max(best, steps + count)
                }
            }
        }

        return worked to best
    }
}