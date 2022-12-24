package day24

val puzzleInput: String = java.io.File("src/main/resources/day24.txt").readText().trim()

data class Pos(val x: Int, val y: Int) {
    fun fourAdjacent() = setOf( Pos(x-1,y), Pos(x+1,y), Pos(x,y-1), Pos(x,y+1))
}

// A quantum of Blizzard.
data class Blizzum(val pos: Pos, val direction: Char)


class Blizzard(val blizzums: Collection<Blizzum>, private val xRange: Int, private val yRange: Int, val round: Int = 0) {

    companion object {

        fun fromString(input: String): Blizzard {
            val height = input.lines().size
            val width = input.lines().first().length

            val blizzums: Set<Blizzum> =
                input.lines().flatMapIndexed { y, line ->
                    line.mapIndexedNotNull { x, entry ->
                        if (entry in setOf('<', '>', '^', 'v')) {
                            Blizzum(Pos(x,y), entry)
                        } else {
                            null
                        }
                    }
                }.toSet()

            return Blizzard(blizzums, width - 2, height - 2)
        }
    }

    operator fun contains(other: Pos): Boolean {
        return blizzums.firstOrNull { it .pos == other } != null
    }

    fun step(): Blizzard {
        val nextBlizzums = blizzums.map { blizzum ->
            val (x, y) = blizzum.pos
            val newPos = when (blizzum.direction) {
                '>' -> Pos((x - 1 + 1).mod(xRange) + 1, y)
                '<' -> Pos((x - 1 - 1).mod(xRange) + 1, y)
                '^' -> Pos(x, (y - 1 - 1).mod(yRange) + 1)
                'v' -> Pos(x, (y - 1 + 1).mod(yRange) + 1)
                else -> error("Unexpected direction symbol")
            }
            Blizzum(newPos, blizzum.direction)
        }

        return Blizzard(nextBlizzums, xRange, yRange, round + 1)
    }

}

class Basin(private var blizzard: Blizzard, private val walkable: Set<Pos>, val entry: Pos, val exit: Pos) {

    private val xRange = 0 .. walkable.maxOf { it.x } + 1
    private val yRange = 0 .. walkable.maxOf { it.y }

    companion object {
        fun fromString(input: String): Basin {
            val blizzard = Blizzard.fromString(input)

            val walkable =
                input.lines().flatMapIndexed { y, line ->
                    line.mapIndexedNotNull { x, entry ->
                        if (entry != '#') Pos(x,y) else null
                    }
                }

            val entry = walkable.first()
            val exit = walkable.last()

            return Basin(blizzard, walkable.toSet(), entry, exit)
        }
    }

    operator fun contains(node: Node): Boolean {
        return when (node.round) {
            blizzard.round -> {
                node.pos in walkable && node.pos !in blizzard
            }
            blizzard.round + 1 -> {
                blizzard = blizzard.step()
                node.pos in walkable && node.pos !in blizzard
            }
            else -> {
                error("Unexpected time step")
            }
        }
    }

    fun draw() {
        val blizzardPos = blizzard.blizzums.map { it.pos }

        println("Round ${blizzard.round}")
        yRange.forEach { y ->
            xRange.forEach { x ->
                when (Pos(x,y)) {
                    !in walkable -> print("#")
                    in blizzardPos -> print("B")
                    else -> print(".")
                }
            }
            println()
        }
        println()
    }
}


data class Node(val pos: Pos, val round: Int) {
    fun neighbours(): List<Node> =
        pos.fourAdjacent().map { Node(it, round + 1) } + Node(pos, round + 1)
}


fun stepsBFS(start: Node, goal: Pos, basin: Basin): Int {

    val seen = mutableSetOf<Node>(start)
    val queue = mutableListOf<Node>(start)

    while (queue.isNotEmpty()) {

        val current = queue.removeFirst()
        val neighbours = current.neighbours()

        if (current.pos == goal) break

        for (nn in neighbours) {
            if (nn !in seen && nn in basin) {
                queue += nn
                seen += nn
            }
        }
    }

    return seen.first { it.pos == goal }.round - start.round
}


fun solvePartOne() {
    val basin = Basin.fromString(puzzleInput)

    val start = Node(basin.entry, 0)
    val goal: Pos = basin.exit

    val steps = stepsBFS(start, goal, basin)
    println("Part 1. Steps = $steps")
}

fun solvePartTwo() {
    val basin = Basin.fromString(puzzleInput)

    val steps1 = stepsBFS( Node(basin.entry, 0),  basin.exit, basin)
    val steps2 = stepsBFS( Node(basin.exit, steps1),  basin.entry, basin)
    val steps3 = stepsBFS( Node(basin.entry, steps1 + steps2),  basin.exit, basin)

    val sum = steps1 + steps2 + steps3
    println("Part 2. Time = $sum")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}
