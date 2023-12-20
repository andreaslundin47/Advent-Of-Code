import kotlin.math.max
import kotlin.math.min

fun main() {
    //val input = java.io.File("src/main/resources/input-19-sample1.txt").readText().trim()
    val input = java.io.File("src/main/resources/input-19.txt").readText().trim()
    val day = Day19(input)

    day.solvePartOne()
    day.solvePartTwo()
}

class Day19(input: String) {

    fun solvePartOne() {
        val sum = parts.sumOf { part ->
            if (isValid(part)) part.value() else 0
        }
        println("Part 1. Sum = $sum")
    }

    fun solvePartTwo() {
        val parts = PartRanges(mapOf(
            Category.X to 1..4000,
            Category.M to 1..4000,
            Category.A to 1..4000,
            Category.S to 1..4000,
        ))

        val flow = workflows.getValue("in")
        val count = flow.validRanges(parts, workflows)

        println("Part 2. Sum = $count")
    }

    fun IntRange.less(value: Int): Pair<IntRange, IntRange> {
        val acc = this.first..min(value-1, this.last)
        val rej = max(value, this.first)..this.last
        return Pair(acc, rej)
    }

    fun IntRange.greater(value: Int): Pair<IntRange, IntRange> {
        val acc = max(value+1, this.first)..this.last
        val rej = this.first..min(value, this.last)
        return Pair(acc, rej)
    }

    enum class Category {
        X,
        M,
        A,
        S,
    }

    enum class Operation {
        LessThan,
        GreaterThan
    }

    private val workflows: Map<String, Workflow> = input
        .split("\n\n")[0]
        .lines()
        .associate { line ->
            val flowName = line.substringBefore("{")
            val rawRules = line.substringAfter("{").trimEnd('}').split(",")
            val elseStatus = rawRules.last()

            val rules = rawRules.dropLast(1).map { rule ->
                val dst = rule.substringAfter(":")
                val comp = rule.substringBefore(":")
                val param = when( comp.take(1).first() ) {
                    'x' -> Category.X
                    'm' -> Category.M
                    'a' -> Category.A
                    's' -> Category.S
                    else -> throw Exception("Bad category letter!")
                }
                val op = when( comp.drop(1).take(1).first()) {
                    '<' -> Operation.LessThan
                    '>' -> Operation.GreaterThan
                    else -> throw Exception("Bad operation symbol")
                }
                val value = comp.drop(2).toInt()
                Rule(param, op, value, dst)
            }

            flowName to Workflow(rules, elseStatus)
        }

    private val parts = input.split("\n\n")[1].lines().map { line ->
        val (x, m, a, s) = line.removePrefix("{").removeSuffix("}").split(',')
        Part(
            mapOf(
                Category.X to x.substringAfter("=").toInt(),
                Category.M to m.substringAfter("=").toInt(),
                Category.A to a.substringAfter("=").toInt(),
                Category.S to s.substringAfter("=").toInt(),
            )
        )
    }.toList()

    data class Part(val values: Map<Category,Int>) {
        fun value(): Int = values.values.sum()
    }

    data class PartRanges(val ranges: Map<Category,IntRange>) {

        fun isEmpty(): Boolean {
            return ranges.values.any { range -> range.last < range.first }
        }

        fun count(): Long {
            return ranges.values.map { it.last - it.first + 1L }.reduce { acc, e -> acc * e }
        }
    }

    data class Workflow(val branches: List<Rule>, val otherwise: String) {
        fun status(part: Part): String {
            for (rule in branches) {
                if (rule.isValid(part)) {
                    return rule.dst
                }
            }
            return otherwise
        }

        fun validRanges(ranges: PartRanges, workflows: Map<String, Workflow>): Long {
            var sum = 0L
            var active = ranges

            for (rule in branches) {
                val (count, rejected) = rule.validRanges(active, workflows)
                sum += count

                if (rejected.isEmpty()) {
                    return sum
                }
                active = rejected
            }

            if (otherwise == "A") {
                sum += active.count()
            }
            else if (otherwise != "R") {
                val flow = workflows.getValue(otherwise)
                sum += flow.validRanges(active, workflows)
            }

            return sum
        }
    }


    inner class Rule(val parameter: Category, val op: Operation, val value: Int, val dst: String) {
        fun isValid(part: Part): Boolean {
            val param = part.values.getValue(parameter)

            return when (op) {
                Operation.LessThan -> param < value
                Operation.GreaterThan -> param > value
            }
        }

        fun validRanges(partRanges: PartRanges, workflows: Map<String, Workflow>): Pair<Long, PartRanges> {
            var sum = 0L

            val (acc, rej) = when (op) {
                Operation.LessThan -> {
                    partRanges.ranges.getValue(parameter).less(value)
                }
                Operation.GreaterThan -> {
                    partRanges.ranges.getValue(parameter).greater(value)
                }
            }

            var cp = partRanges.ranges.toMutableMap()
            cp[parameter] = acc
            val acceptedPartRange = PartRanges(cp)

            if (!acc.isEmpty()) {
                if (dst == "A") {
                    sum += acceptedPartRange.count()
                }
                else if (dst != "R") {
                    val flow = workflows.getValue(dst)
                    sum += flow.validRanges(acceptedPartRange, workflows)
                }
            }

            cp = partRanges.ranges.toMutableMap()
            cp[parameter] = rej
            val rejectedPartRange = PartRanges(cp)


            return Pair(sum, rejectedPartRange)
        }
    }

    private fun isValid(part: Part): Boolean {
        var status = "in"
        while (status !in listOf("A", "R")) {
            val flow = workflows.getValue(status)
            status = flow.status(part)
        }
        return status == "A"
    }
}
