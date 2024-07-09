package day13

// Thanks to Todd Ginsberg for inspiration on improvements!


sealed class Packet: Comparable<Packet> {

    companion object {
        fun fromString(input: Iterator<String>): Packet {
            val packets = mutableListOf<Packet>()
            while (input.hasNext()) {

                when (val symbol = input.next()) {
                    "]" -> return ListPacket(packets)
                    "[" -> packets.add( fromString(input) )
                    else -> packets.add( IntPacket(symbol.toInt()) )
                }
            }

            return ListPacket(packets)
        }

        fun fromString(input: String): Packet {
            val regex = """((?<=[\[,\]])|(?=[\[,\]]))""".toRegex()
            // With this regex we can split the string at all places that come before or
            // after any of the three symbols: '['   ']'   ','
            val tokens = input.split(regex).filter { it != "," }.filter { it.isNotBlank() }
            return fromString(tokens.iterator())
        }
    }
}

class IntPacket(val value: Int): Packet() {

    fun asList(): ListPacket = ListPacket(listOf(this))

    override fun compareTo(other: Packet): Int {
        return when (other) {
            is IntPacket -> value.compareTo(other.value)
            is ListPacket -> asList().compareTo(other)
        }
    }
}

class ListPacket(val packets: List<Packet>): Packet() {

    override fun compareTo(other: Packet): Int {
        return when (other) {
            is IntPacket -> this.compareTo(other.asList())
            is ListPacket -> packets.zip(other.packets)
                .map { (p1, p2) -> p1.compareTo(p2) }
                .firstOrNull { it != 0 } ?: packets.size.compareTo(other.packets.size)

        }
    }
}


val puzzleInput = java.io.File("src/main/resources/day13.txt").readLines()
val packets = puzzleInput.filter { it.isNotBlank() }.map { Packet.fromString(it) }


fun solvePartOne() {
    val sum = packets.chunked(2).mapIndexed { index, pair ->
        if (pair[0] < pair[1]) index + 1 else 0
    }.sum()
    println("Part 1. Sum = $sum")
}

fun solvePartTwo() {
    val dividerPacketOne = Packet.fromString("[[2]]")
    val dividerPacketTwo = Packet.fromString("[[6]]")
    val sorted = (packets + dividerPacketOne + dividerPacketTwo).sorted()
    val key = (sorted.indexOf(dividerPacketOne) + 1) * (sorted.indexOf(dividerPacketTwo) + 1)
    println("Part 2. Product = $key")
}

fun main() {
    solvePartOne()
    solvePartTwo()
}