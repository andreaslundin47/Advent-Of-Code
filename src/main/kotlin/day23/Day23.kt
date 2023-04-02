package day23

val input = java.io.File("src/main/resources/day23.txt").readText().trim()

fun main() {
    println("Part 1. Empty ground tiles: ${Day23(input).solvePartOne()}")
    println("Part 2. Time of equilibrium: ${Day23(input).solvePartTwo()}")
}

data class Point2D(val x: Int, val y: Int) {

    infix operator fun plus(other: Point2D) = Point2D(x + other.x, y + other.y)

    fun neighbours8(): Set<Point2D>
    {
        val x = this.x
        val y = this.y

        return setOf(
            Point2D(x-1,y+1),
            Point2D(x, y+1),
            Point2D(x+1,y+1),
            Point2D(x-1, y),
            Point2D(x+1,y),
            Point2D(x-1, y-1),
            Point2D(x, y-1),
            Point2D(x+1,y-1)
        )
    }
}

class Day23(val input: String) {

    private val startPositions: Set<Point2D> = parseInput(input)
    private val nextTurnOffsets: List<List<Point2D>> = createOffsets()

    fun solvePartOne(): Int {
        var positions = startPositions
        repeat(10) {index -> positions = positions.playRound(index) }
        return positions.rectangleArea() - positions.size
    }

    fun solvePartTwo(): Int {
        var roundNumber = 0
        var thisTurn = startPositions
        do {
            val previousTurn = thisTurn
            thisTurn = thisTurn.playRound(roundNumber++)
        } while (thisTurn != previousTurn)

        return roundNumber
    }

    private fun createOffsets(): List<List<Point2D>> =
        listOf(
            listOf(Point2D(0, -1), Point2D(-1, -1), Point2D(1, -1)),  // North
            listOf(Point2D(0, 1), Point2D(-1, 1), Point2D(1, 1)),     // South
            listOf(Point2D(-1, 0), Point2D(-1, 1), Point2D(-1, -1)),  // West
            listOf(Point2D(1, 0), Point2D(1,  1), Point2D(1, -1))     // East
        )


    private fun parseInput(input: String): Set<Point2D> =
        input.lines().flatMapIndexed { y, row ->
            row.mapIndexedNotNull { x, sym ->
                if (sym == '#') Point2D(x, y) else null
            }
        }.toSet()


    private fun Set<Point2D>.rectangleArea(): Int {
        val dx = this.maxOf { it.x } - this.minOf { it.x } + 1
        val dy = this.maxOf { it.y } - this.minOf { it.y } + 1

        return dx * dy
    }

    private fun Set<Point2D>.playRound(roundIndex: Int = 0): Set<Point2D> {

        // Copy the current positions
        val nextPositions = this.toMutableSet()

        // An elf wants to move if they have at least one neighbour
        val wantsToMove: List<Point2D> = this.filter { elf -> elf.neighbours8().any { it in this } }

        // For each aspiring mover, find if their valid move, if any
        val moversAndDestinations: Map<Point2D, Point2D> =
            wantsToMove
                .mapNotNull { elf ->
                    nextTurnOffsets.indices.map { dirIndex -> nextTurnOffsets[(dirIndex + roundIndex) % 4] }
                        .firstNotNullOfOrNull { offsets ->
                            if (offsets.none {offset -> (elf + offset) in this})
                                elf to (elf + offsets[0])
                            else null
                        }
                }.toMap()

        // Only unique destinations are valid destinations
        val safeDestinations: Set<Point2D> =
            moversAndDestinations.values.groupingBy { it }.eachCount().filter { it.value == 1 }.keys

        // Move all the elves that are able to do so
        moversAndDestinations
            .filter { (_,destination) -> destination in safeDestinations }
            .forEach { (elf, destination) ->
                nextPositions.remove(elf)
                nextPositions.add(destination)
            }

        return nextPositions
   }

}


