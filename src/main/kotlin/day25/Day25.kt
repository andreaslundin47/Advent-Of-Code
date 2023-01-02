package day25

// Thanks to Todd Ginsberg for inspiration on improvements!

fun Long.toSNAFU() =
        generateSequence(this) { (it + 2) / 5 }
            .takeWhile { it > 0 }
            .map { "012=-"[it.mod(5)] }
            .joinToString("")
            .reversed()


fun String.toLong() = fold(0L) { acc, digit -> acc * 5 + "=-012".indexOf(digit) - 2 }

val puzzleInput = java.io.File("src/main/resources/day25.txt").readLines()

fun main() {
    val snafuSum = puzzleInput.sumOf { it.toLong() }.toSNAFU()
    println("Part 1. SNAFU sum = $snafuSum")
}