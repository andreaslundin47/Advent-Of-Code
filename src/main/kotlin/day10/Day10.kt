package day10

val inputs = java.io.File("src/main/resources/day10.txt").readLines()

class CPU(instructions: List<String>) {
    private var register: Int = 1
    val registerHistory = mutableListOf( 0 )

    init {
        instructions.forEach { instruction ->
            if (instruction == "noop") {
                registerHistory.add(register)
            }
            else if (instruction.startsWith("addx")) {
                repeat(2) { registerHistory.add(register) }
                register += instruction.substringAfterLast(" ").toInt()
            }
        }
    }
}

fun drawScreen(registerValues: List<Int>) {
    var cycle = 1
    repeat(6) {
        repeat(40) { column ->
            val register = registerValues[cycle++]
            when (column) {
                register-1, register, register+1 -> print('#')
                else -> print('.')
            }
        }
        println()
    }
}

fun solvePartOne() {
    val cpu = CPU(inputs)

    val signalStrengths = cpu.registerHistory.mapIndexed { cycle, register -> register * cycle }
    val sampleCycles = listOf( 20, 60, 100, 140, 180, 220)
    val signalSum = sampleCycles.sumOf { cycle -> signalStrengths[cycle] }

    println("Part 1. Sum = $signalSum")
}

fun solvePartTwo() {
    val cpu = CPU(inputs)
    println("Part 2.")
    drawScreen(cpu.registerHistory)
}

fun main() {
    solvePartOne()
    solvePartTwo()
}