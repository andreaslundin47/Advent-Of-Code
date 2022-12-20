package day20

val numbers = java.io.File("src/main/resources/day20.txt")
    .readLines().map(String::toInt)


class Number(val value: Long)

fun groveCoordinates(inputSequence: List<Int> , decryptionKey: Int = 1, rounds: Int = 1): List<Long> {

    val initialSequence: List<Number> =
        inputSequence.map { value -> Number( value * decryptionKey.toLong() )
    }

    val length = initialSequence.size
    var mixer = initialSequence

    repeat(rounds) {
        for (numberToMove in initialSequence) {

            val startIndex = mixer.indexOf(numberToMove)
            val finalIndex = (startIndex + numberToMove.value).mod(length - 1)

            val listWithoutMover = mixer.subList(0, startIndex) + mixer.subList(startIndex + 1, length)

            mixer = if (finalIndex > 0) {
                listWithoutMover.subList(0, finalIndex) + numberToMove + listWithoutMover.subList(finalIndex, length - 1)
            } else {
                listWithoutMover.subList(finalIndex, length - 1) + numberToMove
            }
        }
    }

    // Look up the coordinates

    val offsets = listOf(1000, 2000, 3000)
    val zero: Number = initialSequence.single { it.value == 0L }
    val zeroIndex: Int = mixer.indexOf(zero)

    return offsets
        .map { offset -> (zeroIndex + offset).mod(length) }
        .map { mixer[it].value }
}

fun solvePartOne() {
    val coordinates = groveCoordinates(numbers)
    println("Part 1. coordinates = $coordinates   sum = ${coordinates.sum()}")
}

fun solvePartTwo() {
    val coordinates = groveCoordinates(numbers, decryptionKey = 811589153, rounds = 10 )
    println("Part 2. coordinates = $coordinates   sum = ${coordinates.sum()}")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}
