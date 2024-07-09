typealias Grid = List<List<Char>>
typealias Step = Long
typealias Load = Int

fun main() {
    val input = java.io.File("src/main/resources/input-14.txt").readText().trim()

    val day = Day14(input)
    day.solvePartOne()
    day.solvePartTwo()
}

class Day14(input: String) {

    private val inputGrid: Grid = input
        .lines()
            .map { line ->
                line.map { c -> c }
            }


    private fun Grid.clockwise90(): Grid {
        val grid = this.first().indices.map { colIndex ->
            List(this.size) { rowIndex ->
                this[rowIndex][colIndex]
            }.reversed()
        }
        return grid
    }

    private fun Grid.shiftRight(): Grid {
        return this.map { row -> row.shiftRowRight() }.toList()
    }

    private fun Grid.shiftCycle(): Grid {
        var grid = this
        repeat(4) {
            grid = grid.shiftRight()
            grid = grid.clockwise90()
        }
        return grid
    }

    private fun Grid.calculateLoad(): Int {
        return sumOf { row ->
            row.mapIndexed { index, c -> if (c == 'O') index + 1 else 0 }.sum()
        }
    }

    fun Grid.print() {
        println("----------------------------------")
        this.forEachIndexed { _, row ->
            println(row.joinToString(""))
        }
        println("----------------------------------")
    }

    private fun List<Char>.shiftRowRight(): List<Char> {
        val col = this.toMutableList()
        var stillMoving = true

        while (stillMoving) {
            stillMoving = false

            for (i in col.size-1 downTo 1) {
                if (col[i-1] == 'O' && col[i] == '.') {
                    stillMoving = true
                    col[i] = col[i-1]
                    col[i-1] = '.'
                }
            }
        }
        return col
    }

    fun solvePartOne() {
        val grid = inputGrid.clockwise90()
        val shifted = grid.shiftRight()
        val load = shifted.calculateLoad()
        println("Part 1. Load = $load")
    }

    fun solvePartTwo() {
        var grid = inputGrid.clockwise90()
        var step = 0L

        val visited: MutableMap<Grid, Pair<Step, Load>> = mutableMapOf()
        visited[grid] = 0L to grid.calculateLoad()

        while (true) {
            grid = grid.shiftCycle()
            step += 1

            if (grid in visited) {
                break
            } else {
                visited[grid] = step to grid.calculateLoad()
            }
        }

        val (cycleStartStep, _) = visited.getValue(grid)
        val cycleLength = step - cycleStartStep

        val targetStep = 1_000_000_000
        val inCycleOffset = (targetStep - cycleStartStep) % cycleLength
        val answerAtStep = cycleStartStep + inCycleOffset

        val (_, answerLoad) = visited.values.first { (st, _) -> st == answerAtStep }
        println("Part 2. Load = $answerLoad")
    }
}