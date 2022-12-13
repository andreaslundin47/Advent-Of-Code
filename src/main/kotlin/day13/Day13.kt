package day13

val inputPairs: List<Pair<String,String>> = java.io.File("src/main/resources/day13.txt")
    .readText()
    .trim()
    .split("\n\n")
    .map { it.lines().let { (l1, l2) -> l1 to l2 }
}


fun tokenize(signal: String): List<String> {
    var remaining = signal
    val tokens = mutableListOf<String>()

    while (remaining.isNotEmpty()) {

        when (remaining.first()) {
           '[', ']' -> {
               tokens += remaining.first().toString()
               remaining = remaining.drop(1)
           }

           ',' -> {
                remaining = remaining.drop(1)
           }

           in '0'..'9' -> {
                tokens += remaining.takeWhile { it.isDigit() }
                remaining = remaining.dropWhile { it.isDigit() }
            }

           else -> error("Bad!")
        }
    }
    return tokens
}

fun compare(first: List<String>, second: List<String>): Boolean {
    if (first.isEmpty() && second.isNotEmpty()) {
        return true
    }
    else if (first.isNotEmpty() && second.isEmpty()) {
        return false
    }
    else if (first.first() == "]" && second.first() != "]") {
        return true
    }
    else if (first.first() != "]" && second.first() == "]") {
        return false
    }
    else if (first.first() == "[" && second.first() !in setOf("[", "]")) {
        return compare(first, listOf("[", second.first(), "]") + second.drop(1))
    }
    else if (first.first() !in setOf("[", "]") && second.first() == "[") {
        return compare(listOf("[", first.first(), "]") + first.drop(1), second)
    }
    else if (first.first() == second.first()) {
        return compare(first.drop(1), second.drop(1))
    }
    else if (first.first().toInt() < second.first().toInt()) {
        return true
    }
    else if (first.first().toInt() > second.first().toInt()) {
        return false
    }
    else {
        error("Bad!!!")
    }


}


fun solvePartOne() {
   val sum = inputPairs.mapIndexed { i, signals ->
        if (signals.toList().map { tokenize(it) }.let { (s1,s2) -> compare(s1,s2) } ) i+1 else 0
    }.sum()
    println("Part 1. Sum = $sum")
}

fun solvePartTwo() {
    val signals = (inputPairs.flatMap { it.toList() } + listOf( "[[2]]", "[[6]]"))

    val sortedSignals = signals.sortedWith{ s1, s2 ->
        when (compare(tokenize(s1), tokenize(s2)) ) {
            true -> -1
            false -> 1
        }
    }

    val div1 = sortedSignals.indexOf("[[2]]") + 1
    val div2 = sortedSignals.indexOf("[[6]]") + 1
    val prod = div1 * div2

    println("Part 2. Product = $prod")
}


fun main() {
    solvePartOne()
    solvePartTwo()
}