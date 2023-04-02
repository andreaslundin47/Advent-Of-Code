package day21

fun main() {
    println("Part 1. Root yells: ${Day21(input).solvePartOne()}")
    println("Part 1. Human value is: ${Day21(input).solvePartTwo()}")
}

val input = java.io.File("src/main/resources/day21.txt").readLines()

class Day21(val input: List<String>) {

    private val monkeys: Set<Monkey> = parseInput(input)
    private val root: Monkey = monkeys.first { it.name == "root" }

    fun solvePartOne(): Long = root.yell()

    fun solvePartTwo(): Long = root.calculateHumanValue()

    private fun parseInput(input: List<String>): Set<Monkey> {
        val monkeysByName: Map<String, Monkey> = input.map { Monkey.fromString(it) }.associateBy { it.name }

        monkeysByName
            .values
            .filterIsInstance<OperationMonkey>()
            .forEach { monkey ->
                monkey.leftMonkey = monkeysByName.getValue(monkey.leftName)
                monkey.rightMonkey = monkeysByName.getValue(monkey.rightName)
            }

        return monkeysByName.values.toSet()
    }


    interface Monkey {
        val name: String

        fun yell(): Long

        fun calculateHumanValue(humanPath: Set<Monkey> = findHumanPath(), equality: Long = 0L): Long

        fun findHumanPath(): Set<Monkey>

        companion object {
            fun fromString(input: String): Monkey {
                val name = input.substringBefore(":")
                val info = input.substringAfter(": ").split(" ")

                return when (info.size) {
                    1 -> NumberMonkey(name, info[0].toLong())
                    else -> OperationMonkey(name, info[0], info[1].first(), info[2])
                }
            }
        }
    }

    class NumberMonkey(override val name: String, private val number: Long) : Monkey {
        override fun yell(): Long = number

        override fun calculateHumanValue(humanPath: Set<Monkey>, equality: Long): Long =
            if (name == "humn") equality else number

        override fun findHumanPath(): Set<Monkey> =
            if (name == "humn") setOf(this) else emptySet()
    }

    class OperationMonkey(
        override val name: String,
        val leftName: String,
        private val op: Char,
        val rightName: String
    ) : Monkey {

        lateinit var leftMonkey: Monkey
        lateinit var rightMonkey: Monkey

        override fun yell(): Long = leftMonkey.yell() operation rightMonkey.yell()

        override fun calculateHumanValue(humanPath: Set<Monkey>, equality: Long): Long {
            return if (name == "root") {
                if (leftMonkey in humanPath)
                    leftMonkey.calculateHumanValue(humanPath, rightMonkey.yell())
                else
                    rightMonkey.calculateHumanValue(humanPath, leftMonkey.yell())
            }
            else if (leftMonkey in humanPath) {
                leftMonkey.calculateHumanValue(humanPath, equality leftInverseOp rightMonkey.yell())
            }
            else {
                rightMonkey.calculateHumanValue(humanPath, equality rightInverseOp leftMonkey.yell())
            }
        }

        override fun findHumanPath(): Set<Monkey> {
            val left = leftMonkey.findHumanPath()
            val right = rightMonkey.findHumanPath()

            return if (left.isNotEmpty()) left + this
            else if (right.isNotEmpty()) right + this
            else emptySet()
        }

        private infix fun Long.operation(right: Long): Long =
            when (op) {
                '+' -> this + right
                '-' -> this - right
                '*' -> this * right
                '/' -> this / right
                else -> error("Invalid math operation")
            }

        private infix fun Long.leftInverseOp(right: Long): Long =
            when (op) {
                '+' -> this - right
                '-' -> this + right
                '*' -> this / right
                else -> this * right
            }

        private infix fun Long.rightInverseOp(right: Long): Long =
            when (op) {
                '+' -> this - right
                '-' -> right - this
                '*' -> this / right
                else -> right / this
            }
    }
}


