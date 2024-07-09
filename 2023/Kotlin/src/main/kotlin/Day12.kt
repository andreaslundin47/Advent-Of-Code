import kotlin.math.sign

class Day12(input: String) {
    class Record(val record: String, val blockSizes: List<Int>) {

        data class State(val start: Int, val blocks: List<Int>, val intervals: List<IntRange>)

        private val memory: MutableMap<State, Long> = mutableMapOf()

        private fun countCombinations(record: String, blocks: List<Int>): Long {
            if (blocks.isEmpty()) {
                return if ('#' !in record) { 1L } else { 0L }
            }

            return blocks.dropLast(1).sumOf { it.sign }.toLong()
        }

        private fun count(state: State): Long {

            if (state in memory) {
                return memory.getValue(state)
            }

            val (windowStart, blocks, intervals) = state

            // Success! A valid combination
            if (blocks.isEmpty()) {
                return if (windowStart >= record.length) {
                    1L
                } else {
                    val clean = '#' !in record.substring(windowStart, record.length)
                    if (clean) 1L else 0L
                }
            }

            val block = blocks.first()
            val interval = intervals.first()

            var sum: Long = 0L

            for (start in interval) {

                if (start + block > record.length) {
                    return 0L
                }

                if (start < windowStart) {
                    continue
                }

                if ('#' in record.substring(windowStart, start)) {
                    continue
                }
                if ('.' in record.substring(start, start + block)) {
                    continue
                }
                if (start+block < record.length && record[start+block] == '#') {
                    continue
                }

                val newState = State(start+block+1, blocks.drop(1), intervals.drop(1))
                val result: Long = count(newState)
                sum += result
            }

            memory[state] = sum

            return sum
        }

        fun combinations(): Long {
            val lefts = mutableListOf<Int>()
            var counter = 0
            for (bs in blockSizes) {
                lefts += counter
                counter += bs + 1
            }

            val rights = mutableListOf<Int>()
            counter = record.length - (blockSizes.sum() + blockSizes.size - 1)
            for (bs in blockSizes) {
                rights += counter
                counter += (bs + 1)
            }

            val startingIntervals: List<IntRange> = lefts.zip(rights).map { (a,b) -> a..b }

            val state = State(0, blockSizes, startingIntervals)

            return count(state)
        }
    }

    val records: List<Record> =input
        .lines()
        .map { line ->
            val (record, sizes) = line.split(" ")
            val blocks = sizes.split(",").map { s -> s.toInt() }
            Record(record, blocks)
    }

    fun partTwo() {
        var sum = 0L
        for ((i, r) in records.withIndex()) {
            val rrr = List(5) { r.record }.joinToString("?")
            val bb = List(5) { r.blockSizes }.flatten()
            val bigRecord = Record(rrr, bb)

            val count = bigRecord.combinations()
            // println("Record ${i+1} has $count combinations")
            sum += count
        }

        println("Part 2. Sum of combinations = $sum")
    }
    fun partOne() {

        var sum = 0L
        for ((i, r) in records.withIndex()) {
            val count = r.combinations()
            //println("Record ${i+1} has $count combinations")
            sum += count
        }

        println("Part 1. Sum of combinations = $sum")
    }
}

fun main() {
    val input = java.io.File("src/main/resources/input-12.txt").readText().trim()
    val day = Day12(input)
    day.partOne()
    day.partTwo()
}