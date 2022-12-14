package day13

data class Signal(val signal: String): Comparable<Signal> {

    private val tokenized: List<String> = tokenize(signal)

    private fun tokenize(signal: String): List<String> {
        var remaining = signal
        val tokens = mutableListOf<String>()

        while (remaining.isNotEmpty()) {

            when (remaining.first()) {
                '[', ']' -> {
                    tokens += remaining.first().toString()
                    remaining = remaining.drop(1)
                }

                ',' -> remaining = remaining.drop(1)

                in '0'..'9' -> {
                    tokens += remaining.takeWhile { it.isDigit() }
                    remaining = remaining.dropWhile { it.isDigit() }
                }

                else -> error("Unexpected character while parsing signal representation")
            }
        }
        return tokens
    }

    override fun compareTo(other: Signal): Int {
        return compare(this.tokenized, other.tokenized)
    }


    private fun compare(list1: List<String>, list2: List<String>): Int {

        fun String.isNumber() = this !in setOf("[", "]")

        fun List<String>.makeHeadList() = listOf("[", this.first(), "]") + this.drop(1)

        fun compareRec(l1: List<String>, l2: List<String>): Int {
            if (l1.isEmpty() && l2.isNotEmpty())
                return -1

            else if (l1.isNotEmpty() && l2.isEmpty())
                return 1

            else if (l1.first() == l2.first())
                return compareRec(l1.drop(1), l2.drop(1))

            else if (l1.first() == "]" && l2.first() != "]")
                return -1

            else if (l2.first() == "]" && l1.first() != "]")
                return 1

            else if (l1.first().isNumber() && l2.first() == "[")
                return compareRec(l1.makeHeadList(), l2)

            else if (l1.first() == "[" && l2.first().isNumber())
                return compareRec(l1, l2.makeHeadList())

            else if (l1.first().toInt() < l2.first().toInt())
                return -1

            else if (l1.first().toInt() > l2.first().toInt())
                return 1

            else
                error("Unexpected list pair in comparison")
        }

        return compareRec(list1, list2)
    }
}


val signalPairs: List<Pair<Signal,Signal>> = java.io.File("src/main/resources/day13.txt")
    .readText()
    .trim()
    .split("\n\n")
    .map { block -> block.lines().let { (l1, l2) -> Signal(l1) to Signal(l2) }
}

fun solvePartOne() {
    val sum = signalPairs.mapIndexed { i, signals ->
        signals.let { (s1, s2) -> if (s1 < s2) i+1 else 0 }
    }.sum()
    println("Part 1. Sum = $sum")
}

fun solvePartTwo() {
    val divs = listOf( Signal("[[2]]"), Signal("[[6]]") )
    val signalsAndDivs = divs + signalPairs.flatMap { it.toList() }
    val sortedSignals = signalsAndDivs.sorted()
    val prod = divs.map { sortedSignals.indexOf(it) + 1 }.reduce(Int::times)
    println("Part 2. Product = $prod")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}