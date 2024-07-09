package day07

val input = java.io.File("src/main/resources/day07.txt").readLines()

data class File(val name: String, val size: Int)

data class Directory(val name: String,
                     val files: MutableSet<File>,
                     val directories: MutableMap<String,Directory>,
                     val parent: Directory?)
{
    companion object {
        fun createEmpty(name: String, parent: Directory?) =
            Directory(name, mutableSetOf<File>(), mutableMapOf<String,Directory>(), parent)

        fun directorySizes(root: Directory): List<Int> {
            val sizes = mutableListOf<Int>()

            fun dirSize(dir: Directory): Int {
                val sizeOfFiles = dir.files.sumOf { it.size }
                val sizeOfSubDirectories = dir.directories.map { (_, subDir) -> dirSize(subDir) }.sum()
                val totalSize = sizeOfFiles + sizeOfSubDirectories
                sizes.add(totalSize)
                return totalSize
            }
            dirSize(root)
            return sizes
        }
    }
}


fun build(input: List<String>): Directory {
    val root = Directory.createEmpty("/", null)
    var cd = root

    fun String.isInt(): Boolean = toIntOrNull() != null

    for (line in input) {
        when {
            line.startsWith("$ ls") -> Unit
            line.startsWith("$ cd /") -> cd = root
            line.startsWith("$ cd ..") -> cd = cd.parent ?: root
            line.startsWith("$ cd") -> {
                val dir = line.substringAfter("cd ")
                cd = cd.directories[dir] ?: error("Tried to enter non-existing directory")
            }
            line.startsWith("dir") -> {
                val dirName = line.substringAfter("dir ")
                cd.directories.putIfAbsent(dirName, Directory.createEmpty(dirName, cd))
            }
            line.substringBefore(" ").isInt() -> {
                val size = line.substringBefore(" ").toInt()
                val name = line.substringAfter(" ")
                cd.files.add( File(name, size) )
            }
            else -> error("Unexpected input line")
        }
    }

    return root
}

fun solvePartOne() {
    val filesystem: Directory = build(input)
    val sumPartOne = Directory.directorySizes(filesystem).filter { it <= 100000 }.sum()
    println("Part 1. $sumPartOne")
}

fun solvePartTwo() {
    val filesystem: Directory = build(input)
    val directorySizes: List<Int> = Directory.directorySizes((filesystem))

    val totalMemory = 70_000_000
    val usedMemory = directorySizes.max()
    val freeMemory = totalMemory - usedMemory

    val freeMemoryNeededForUpdate = 30_000_000
    val memoryNeededToFreeUp = freeMemoryNeededForUpdate - freeMemory

    // Delete the smallest directory that is large enough
    val sizeToDelete = directorySizes.filter { it >= memoryNeededToFreeUp }.min()
    println("Part 2. $sizeToDelete")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}