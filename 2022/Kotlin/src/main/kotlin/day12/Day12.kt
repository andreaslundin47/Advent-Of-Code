package day12

val inputs = java.io.File("src/main/resources/day12.txt").readText()

interface Node

data class Pos(val x: Int, val y: Int): Node {

    fun neighbours(): Set<Pos> = setOf( Pos(x-1, y), Pos(x+1, y), Pos(x, y-1), Pos(x, y+1) )
}

fun heightFromLabel(c: Char): Int =
    when (c) {
        'S' -> 0
        'E' -> 25
        else -> c - 'a'
    }

fun getLabelMapping(inputs: String): Map<Pos,Char> {
    val grid = inputs.trim().lines().map { it.toList() }

    val labelMapping = grid.flatMapIndexed { j, row ->
        row.mapIndexed { i, label ->
            Pos(i,j) to label
        }
    }.toMap()

    return labelMapping
}


fun breadthFirstSearchShortestPath(start: Node,
                                   goals: Collection<Node>,
                                   neighbours: Map<Pos,Collection<Node>>
): List<Node> {
    val seen = mutableSetOf( start )
    val previous = mutableMapOf<Node,Node?>( start to null )

    val queue = mutableListOf( start )

    while (queue.isNotEmpty()) {
        val current = queue.removeFirst()
        if (current in goals) break

        val reachable = neighbours[current] ?: emptySet()

        for (neighbour in reachable) {
            if (neighbour !in seen) {
                previous[neighbour] = current
                queue.add(neighbour)
                seen.add (neighbour)
            }
        }
    }

    val path = mutableListOf<Node>()
    var current = previous.keys.firstOrNull() { it in goals }

    while (current != null) {
        path.add(current)
        current = previous[current]
    }

    return path.reversed()
}




fun solvePartOne() {
    val labelMapping = getLabelMapping(inputs)
    val heightMapping = labelMapping.mapValues { heightFromLabel(it.value) }
    val start = labelMapping.filterValues { it == 'S' }.keys.first()
    val goal = labelMapping.filterValues { it == 'E' }.keys.first()

    fun upHillNeighbours(pos: Pos, height: Int) =
        pos.neighbours().filter { np -> (heightMapping[np] ?: 0) <= height + 1 }.toSet()

    val neighbours = heightMapping.mapValues { (pos, height) -> upHillNeighbours(pos, height) }

    val path = breadthFirstSearchShortestPath(start, setOf(goal), neighbours)
    val distance = path.size - 1

    println("Part 1. $distance")
}


fun solvePartTwo() {
    val labelMapping = getLabelMapping(inputs)
    val heightMapping = labelMapping.mapValues { heightFromLabel(it.value) }
    val start = labelMapping.filterValues { it == 'E' }.keys.first()
    val goals = labelMapping.filterValues { it in setOf('S', 'a') }.keys

    fun downHillNeighbours(pos: Pos, height: Int) =
        pos.neighbours().filter { np -> (heightMapping[np] ?: 0) >= height - 1 }.toSet()

    val neighbours = heightMapping.mapValues { (pos, height) -> downHillNeighbours(pos, height) }

    val path = breadthFirstSearchShortestPath(start, goals, neighbours)
    val distance = path.size - 1

    println("Part 2. $distance")
}


fun main() {
    solvePartOne()
    solvePartTwo()
}