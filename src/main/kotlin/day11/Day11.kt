package day11

val inputs = java.io.File("src/main/resources/day11.txt").readText().trim().split("\n\n")

typealias AmplifierFunction = (Long) -> Long

fun toAmplifierFunction(text: String): AmplifierFunction {
    val (_, op, right) = text.split(" ")
    val rr = right.toIntOrNull()

    return when (op) {
        "+" -> if (rr != null) { x -> x + rr } else { x -> x + x }
        "*" -> if (rr != null) { x -> x * rr } else { x -> x * x }
        else -> error("Unexpected string representation")
    }
}


data class MonkeyData(val items: List<Long>,
                      val worryAmplifier: AmplifierFunction,
                      val divisor: Long,
                      val targetTrue: Int,
                      val targetFalse: Int,
) {
    companion object {
        fun fromString(text: String): MonkeyData {
            val rows = text.lines()

            val items = rows[1].substringAfter(": ").split(", ").map { it.toLong() }
            val amplifier = toAmplifierFunction(rows[2].substringAfterLast("= "))
            val divisor = rows[3].substringAfterLast(" ").toLong()
            val targetTrue = rows[4].substringAfterLast(" ").toInt()
            val targetFalse = rows[5].substringAfterLast(" ").toInt()

            return MonkeyData(items, amplifier, divisor, targetTrue, targetFalse)
        }
    }
}


fun solvePartOne() {
    val monkeys = inputs.map { MonkeyData.fromString(it) }
    val monkeyItems: List<MutableList<Long>> = monkeys.map { it.items.toMutableList() }
    val inspectionCounts = MutableList(monkeys.size) { 0 }


    repeat(20) {
        monkeys.forEachIndexed { i, monkey ->

            monkeyItems[i].forEach { item ->
                val worry = monkey.worryAmplifier(item) / 3

                when (worry % monkey.divisor == 0L) {
                    true -> monkeyItems[monkey.targetTrue] += worry
                    false -> monkeyItems[monkey.targetFalse] += worry
                }
            }

            inspectionCounts[i] += monkeyItems[i].size
            monkeyItems[i].clear()
        }
    }

    val monkeyBusiness = inspectionCounts.sorted().takeLast(2).let { (a,b) -> a * b }
    println("Part 1. Monkey Business = $monkeyBusiness")
}


fun solvePartTwo() {
    val monkeys = inputs.map { MonkeyData.fromString(it) }
    val monkeyItems: List<MutableList<Long>> = monkeys.map { it.items.toMutableList() }
    val inspectionCounts = MutableList<Long>(monkeys.size) { 0 }

    val gcd = monkeys.map { it.divisor }.reduce { acc, v -> acc * v }

    repeat(10_000) {
        monkeys.forEachIndexed { i, monkey ->

            monkeyItems[i].forEach { item ->
                val worry = monkey.worryAmplifier(item) % gcd

                when (worry % monkey.divisor == 0L) {
                    true -> monkeyItems[monkey.targetTrue] += worry
                    false -> monkeyItems[monkey.targetFalse] += worry
                }
            }

            inspectionCounts[i] += monkeyItems[i].size.toLong()
            monkeyItems[i].clear()
        }
    }

    val monkeyBusiness = inspectionCounts.sorted().takeLast(2).let { (a,b) -> a * b }
    println("Part 2. Monkey Business = $monkeyBusiness")
}


fun main() {
    solvePartOne()
    solvePartTwo()
}