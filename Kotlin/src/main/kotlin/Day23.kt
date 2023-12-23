import java.util.PriorityQueue
import kotlin.math.absoluteValue

fun main() {
    val input = java.io.File("src/main/resources/input-23.txt").readText().trim()
    //val input = java.io.File("src/main/resources/input-23-sample1.txt").readText().trim()
    val day = Day23(input)
    day.solvePartOne()
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

    fun Point.IsExit(other: Point): Boolean {
        if (!isAdjacent(other) || other !in points) {
            return false
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
            .associateWith { entry -> entry.neighbours().filter { n -> entry.IsExit(n) } }
            .toMutableMap() +
                (start to listOf(start + south))
            ).toMap()

    private val edges: Map<Point, Set<Edge>> = entryPoints
        .map { (entry, exits) ->
            entry to exits.map { ex -> entry.findEdge(ex) }.toSet()
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
         val count = intersections.size
         println("Part 1. Vertices = $count")
         println("Part 1. Longest distance = $longest")
    }

}