package day11

fun main() {
    val inputs = java.io.File("src/main/resources/day11.txt").readText()

    println("Part 1. ${Day11(inputs).solvePartOne()}")
    println("Part 2. ${Day11(inputs).solvePartTwo()}")
}


class Day11(input: String) {

    private val monkeys = input.trim().split("\n\n").map { Monkey.fromString(it.lines()) }

    fun solvePartOne(): Long {
        playRounds(20) { it / 3 }
        return monkeys.business()
    }

    fun solvePartTwo(): Long {
        val factor = monkeys.map { it.testDivisor }.reduce(Long::times)
        playRounds(10_000) { it % factor }
        return monkeys.business()
    }

    private fun playRounds(rounds: Int, worryPacifier: (Long) -> Long) {
        repeat(rounds) {
            monkeys.forEach { it.inspectItems(monkeys, worryPacifier) }
        }
    }

    private fun List<Monkey>.business(): Long =
        map { it.interactions }.sortedDescending().take(2).reduce(Long::times)

    private class Monkey(
        val items: MutableList<Long>,
        val operation: (Long) -> Long,
        val testDivisor: Long,
        val trueMonkeyId: Int,
        val falseMonkeyId: Int
    ) {
        var interactions: Long = 0

        fun inspectItems(monkeys: List<Monkey>, worryPacifier: (Long) -> Long) {

            items.forEach { item ->
                val worry = worryPacifier( operation(item) )
                val target = if (worry % testDivisor == 0L) trueMonkeyId else falseMonkeyId
                monkeys[target].items.add(worry)
            }

            interactions += items.size
            items.clear()
        }

        companion object {

            fun parseOperation(input: String): (Long) -> Long {
                val (_, op, right) = input.split(" ")
                val ll = right.toLongOrNull()

                return when (op) {
                    "+" -> if (ll == null) ({ it + it }) else ({ it + ll })
                    else -> if (ll == null) ({ it * it }) else ({ it * ll })
                }
            }

            fun fromString(input: List<String>): Monkey {

                val items = input[1]
                    .substringAfter(":")
                    .split(",")
                    .map { it.trim().toLong() }
                    .toMutableList()

                val operation = parseOperation(input[2].substringAfter("= "))
                val divisor = input[3].substringAfter("by ").toLong()
                val trueMonkey = input[4].substringAfter("monkey ").toInt()
                val falseMonkey = input[5].substringAfter("monkey ").toInt()

                return Monkey(items, operation, divisor, trueMonkey, falseMonkey)
            }
        }
    }
}


