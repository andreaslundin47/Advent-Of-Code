package day25

import kotlin.math.pow

val puzzleInput = java.io.File("src/main/resources/day25.txt").readLines()


class Decimal(private val value: Long) {
    override fun toString() = value.toString()
    operator fun plus(other: Decimal) = Decimal(value + other.value)

    fun toSNAFU(): SNAFU {
        var snafu = ""
        var remaining = value

        while (remaining > 0L) {

            val (symbol, delta) =
                when (remaining.mod(5)) {
                    0 -> '0' to 0
                    1 -> '1' to 1
                    2 -> '2' to 2
                    3 -> '=' to -2
                    4 -> '-' to -1
                    else -> error("Strange SNAFU string representation")
                }

            snafu = symbol + snafu
            remaining = (remaining - delta) / 5
        }

        return SNAFU(snafu)
    }
}


class SNAFU(private val value: String) {
    override fun toString() = value

    fun toDecimal(): Decimal {
        val decimalValue =
            value.reversed().mapIndexed { index, symbol ->
                5.0.pow(index) * when (symbol) {
                    '0' -> 0
                    '1' -> 1
                    '2' -> 2
                    '=' -> -2
                    '-' -> -1
                    else -> error("Strange digit")
                }
            }.sum().toLong()

        return Decimal(decimalValue)
    }
}


fun solvePartOne() {
    val snafuList = puzzleInput.map { snafuString -> SNAFU(snafuString) }
    val decimalList = snafuList.map { snafu -> snafu.toDecimal() }
    val decimalSum = decimalList.fold(Decimal(0)) { acc, num -> acc + num }
    val snafuSum = decimalSum.toSNAFU()

    println("Part 1. SNAFU sum = $snafuSum")
}

fun main() {
    solvePartOne()
}