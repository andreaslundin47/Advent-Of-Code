class Day03(input: String) {

    data class Vec2(val x: Int, val y: Int)

    data class Symbol(val type: Char, val pos: Vec2)

    data class Number(val value: Int, val start: Vec2, val length: Int) {

        private val xRange: IntRange = (start.x - 1)..(start.x + length)
        private val yRange: IntRange = (start.y - 1)..(start.y + 1)

        fun contains(pos: Vec2): Boolean = pos.x in xRange && pos.y in yRange
    }

    private val numbers = mutableSetOf<Number>()
    private val symbols = mutableSetOf<Symbol>()

    private fun parseLine(line: String, rowIndex: Int) {
        var index = 0

        var remaining = line
        while (remaining != "") {
            if (remaining[0].isDigit()) {
                val a = remaining.takeWhile { c -> c.isDigit() }
                numbers += Number(a.toInt(), Vec2(index, rowIndex), a.length)

                remaining = remaining.drop(a.length)
                index += a.length
            }
            else {
                val c = remaining[0]

                if (c != '.') {
                    val coord = Vec2(index, rowIndex)
                    symbols += Symbol(c, coord)
                }

                remaining = remaining.drop(1)
                index += 1
            }
        }
    }

    init {
        input.lines().forEachIndexed { ind, line ->
            parseLine(line, ind)
        }
    }

    fun solvePartOne() {
        //val (numbers, symbols) = parseInput(input)
        val sum = numbers
            .filter { number ->
                symbols.any { symbol ->
                    number.contains(symbol.pos)
                }
            }
            .sumOf { it.value }

        println("Part 1. Sum = $sum")
    }

    fun solvePartTwo() {
        val sum = symbols
            .filter { it.type == '*' }
            .map { gear ->
                numbers.filter { it.contains(gear.pos) }
            }
            .filter { adjNumbers ->
                adjNumbers.size == 2
            }
            .sumOf { pair -> pair.map { it.value }.reduce(Int::times) }

        println("Part 2. Sum = $sum")
    }
}

fun main() {
    val rawInput = java.io.File("src/main/resources/input-03.txt").readText().trim()
    val d = Day03(rawInput)
    d.solvePartOne()
    d.solvePartTwo()
}