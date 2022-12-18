package day18

val lavaDroplets: Set<Vec3> = java.io.File("src/main/resources/day18.txt")
    .readLines()
    .map { entry ->
        entry.split(",").map(String::toInt).let { (x,y,z) -> Vec3(x,y,z) }
    }.toSet()

data class Vec3(val x: Int, val y: Int, val z: Int) {
    fun sixNeighbours(): List<Vec3> =
         listOf(
             Vec3(x-1, y, z), Vec3(x+1, y, z),
             Vec3(x,y-1, z), Vec3(x, y+1, z),
             Vec3(x, y, z-1), Vec3(x, y, z+1)
        )
}

fun solvePartOne() {
    val surfaceCount = lavaDroplets.sumOf { droplet ->
        droplet.sixNeighbours().count { it !in lavaDroplets }
    }
    println("Part 1. Number of lava to non-lava surfaces = $surfaceCount")
}

fun solvePartTwo() {
    // Define an enclosing volume with some padding

    val xRange = lavaDroplets.minOf { it.x } - 1 .. lavaDroplets.maxOf { it.x } + 1
    val yRange = lavaDroplets.minOf { it.y } - 1 .. lavaDroplets.maxOf { it.y } + 1
    val zRange = lavaDroplets.minOf { it.z } - 1 .. lavaDroplets.maxOf { it.z } + 1


    // Breadth First Search within volume to find all unit volumes reachable from the exterior.
    // We start search from a node inside the padding region
    val start = Vec3(xRange.first, yRange.first, zRange.first)

    val queue = mutableListOf(start)
    val exteriorVolumes = mutableSetOf(start)

    fun Vec3.isInsideVolume(): Boolean = x in xRange && y in yRange && z in zRange

    while (queue.isNotEmpty()) {
        val currentNode = queue.removeFirst()
        val neighboursWithinVolume = currentNode.sixNeighbours().filter { it.isInsideVolume() }

        for (neighbour in neighboursWithinVolume) {
            if (neighbour !in lavaDroplets && neighbour !in exteriorVolumes) {
                queue += neighbour
                exteriorVolumes += neighbour
            }
        }
    }

    // Find all unit volumes containing air bubbles
    val allEnclosingVolumes = xRange.flatMap { x -> yRange.flatMap { y -> zRange.map { z -> Vec3(x,y,z) } } }.toSet()
    val airBubbles = allEnclosingVolumes - exteriorVolumes - lavaDroplets

    val lavaSurfaceCount = lavaDroplets.sumOf { droplet ->
        droplet.sixNeighbours().count { it !in lavaDroplets }
    }

    val airToLavaSurfaceCount = airBubbles.sumOf { bubble ->
        bubble.sixNeighbours().count { it in lavaDroplets }
    }

    val lavaToExteriorSurfaceCount = lavaSurfaceCount - airToLavaSurfaceCount

    println("Part 2. Number of lava to non-lava, exterior surfaces = $lavaToExteriorSurfaceCount")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}
