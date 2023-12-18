import kotlin.math.max
import kotlin.math.min

fun main() {
    val input = java.io.File("src/main/resources/input-18.txt").readText().trim().lines()
    val day = Day18(input)
    day.solvePartOne()
    day.solvePartTwo()
}

class Day18(private val input: List<String>) {
    data class DigStep(val delta: Node)
    data class Node(val x: Long, val y: Long) {
        operator fun plus(other: Node): Node {
            return Node(x + other.x, y + other.y)
        }
        fun neighbours(): Set<Node> {
            val (x, y) = this
            return listOf(
                Node(x+1,y),
                Node(x-1,y),
                Node(x,y+1),
                Node(x,y-1)
            ).toSet()
        }
    }

    fun solvePartOne() {
        val steps = parseOne(input)
        println("Part 1. Size = ${digLagoon(steps)}")
    }

    fun solvePartTwo() {
        val steps = parseTwo(input)
        println("Part 2. Size = ${digLagoon(steps)}")
    }

    private fun decodeHex(hex: String): Long {
        val base10 = hex.fold(0L) { acc, d ->
            (16 * acc) + "0123456789abcdef".indexOf(d)
        }
        return base10
    }

    private fun parseOne(input: List<String>): List<DigStep> {
        return input
            .map { line ->
                val (rawDir, dist, _) = line.split(" ")

                val dir = when(rawDir.first()) {
                    'U' -> Node(0L, -dist.toLong())
                    'D' -> Node(0L, dist.toLong())
                    'L' -> Node(-dist.toLong(), 0)
                    'R' -> Node(dist.toLong(), 0)
                    else -> throw Exception("Bad Direction!")
                }
                DigStep(dir)
            }
    }

    private fun parseTwo(input: List<String>): List<DigStep> {
        return input
            .map { line ->
                val content = line
                    .substringAfterLast(" ")
                    .removePrefix("(#")
                    .removeSuffix(")")

                val dist: Long = decodeHex(content.take(5))
                val dirNum = content.drop(5).toInt()

                val dir = when(dirNum) {
                    0 -> Node(dist, 0L)
                    1 -> Node(0L, dist)
                    2 -> Node(-dist, 0L)
                    3 -> Node(0L, -dist)
                    else -> throw Exception("Bad Direction!")
                }
                DigStep(dir)
            }
    }

    private fun digLagoon(steps: List<DigStep>): Long {

        // Find all coordinates where boundary segments begin and end.
        // Boundary is 1 element thick, so collect them plus 1 as well.
        val xCoordinates = mutableSetOf<Long>()
        val yCoordinates = mutableSetOf<Long>()

        var current = Node(0, 0)
        for (step in steps) {
            current += step.delta
            xCoordinates += current.x
            xCoordinates += (current.x + 1)
            yCoordinates += current.y
            yCoordinates += (current.y + 1)
        }

        // From ordered lists of all unique x and y coordinates, create a mapping to indices
        val uniqueX: List<Long> = xCoordinates.sorted()
        val uniqueY: List<Long> = yCoordinates.sorted()

        // From world to indices
        val worldXToInd: Map<Long, Long> = uniqueX.mapIndexed { index, x -> x to index.toLong() }.toMap()
        val worldYToInd: Map<Long, Long> = uniqueY.mapIndexed { index, y -> y to index.toLong() }.toMap()

        // From indices back to world coordinates
        val xIndexToWorld = worldXToInd.map { (k, v) -> v to k }.toMap()
        val yIndexToWorld = worldYToInd.map { (k, v) -> v to k }.toMap()

        // Find all boundary nodes, mapped to index space
        val boundary = mutableSetOf<Node>()
        current = Node(0, 0)

        for (step in steps) {
            val previous = current
            current += step.delta

            if (step.delta.x != 0L) {
                val y = worldYToInd.getValue(previous.y)

                val left = worldXToInd.getValue(min(previous.x, current.x))
                val right = worldXToInd.getValue(max(previous.x, current.x))

                for (x in left..right) {
                    boundary += Node(x, y)
                }
            } else {
                val x = worldXToInd.getValue(previous.x)

                val bottom = worldYToInd.getValue(min(previous.y, current.y))
                val top = worldYToInd.getValue(max(previous.y, current.y))

                for (y in bottom..top) {
                    boundary += Node(x, y)
                }
            }
        }

        // Use BFS to find all interior nodes enclosed by the boundary
        val interior = interiorFromBoundary(boundary)

        val lagoonNodes: Set<Node> = interior + boundary

        // Map back each index node to world space, and sum their sizes
        val lagoonSize = lagoonNodes.sumOf { node ->
            val (xIndex, yIndex) = node
            val dx = xIndexToWorld.getValue(xIndex+1) - xIndexToWorld.getValue(xIndex)
            val dy = yIndexToWorld.getValue(yIndex+1) - yIndexToWorld.getValue(yIndex)
            dx * dy
        }

        return lagoonSize
    }

    private fun interiorFromBoundary(boundary: Set<Node>): Set<Node> {

        val minX = boundary.minOf { b -> b.x }

        val interiorNodeAdjacentBoundary = boundary
            .filter { b ->
                b.x == minX
            }
            .first { b ->
                (b + Node(1, 0)) !in boundary
            }

        val interiorStart = interiorNodeAdjacentBoundary + Node(1, 0)

        val queue = mutableListOf(interiorStart)
        val seen = mutableSetOf(interiorStart)

        while (queue.isNotEmpty()) {
            val curr = queue.removeFirst()

            for (neighbour in curr.neighbours()) {
                if (neighbour !in boundary && neighbour !in seen) {
                    queue += neighbour
                    seen += neighbour
                }
            }
        }

        return seen
    }
}
