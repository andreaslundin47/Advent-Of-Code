fun main() {
    val input = java.io.File("src/main/resources/input-20.txt").readText().trim()
    val day = Day20(input)
    day.solvePartOne()
}
class Day20(private val input: String) {

    fun solvePartOne() {
        val units = parseUnits(input)
        val sentPackets = mutableListOf<Packet>()

        var lows = 0
        var highs = 0

        repeat(1000) {
            sentPackets += Packet("broadcaster", "button", Pulse.Low)
            while (sentPackets.isNotEmpty()) {
                val packet = sentPackets.removeAt(0)

                when (packet.pulse) {
                    Pulse.High -> highs += 1
                    Pulse.Low -> lows += 1
                }

                if (packet.destination in units) {
                    val currentUnit = units.getValue(packet.destination)
                    val newPackets = currentUnit.processPacket(packet)
                    sentPackets.addAll(newPackets)
                }
            }
        }

        val prod = lows * highs
        println("Part 1. Prod = $prod")
    }

    enum class Pulse {
        Low,
        High
    }

    enum class UnitType {
        FlipFlop,
        Conjunction,
        BroadCaster
    }

    data class Packet(val destination: String, val source: String, val pulse: Pulse)

    sealed class Unit() {
        abstract fun processPacket(packet: Packet): List<Packet>
    }

    class Broadcaster(val name: String, private val receivers: List<String>): Unit() {
        override fun processPacket(packet: Packet): List<Packet> {
            return receivers.map { Packet(it, name, packet.pulse) }
        }
    }

    class FlipFlop(val name: String, private val receivers: List<String>, private var turnedOn: Boolean = false): Unit() {
        override fun processPacket(packet: Packet): List<Packet> {
            return if (packet.pulse == Pulse.High) {
                emptyList()
            } else {
                turnedOn = !turnedOn
                val outPulse = when (turnedOn) {
                    true -> Pulse.High
                    false -> Pulse.Low
                }
                receivers.map { Packet(it, name, outPulse) }
            }
        }
    }

    class Conjunction(val name: String, private val receivers: List<String>, senders: List<String>): Unit() {

        private val senders: MutableMap<String, Pulse> = senders
            .associateWith { Pulse.Low }
            .toMutableMap()

        override fun processPacket(packet: Packet): List<Packet> {
            senders[packet.source] = packet.pulse
            val outPulse = when (senders.values.all { it == Pulse.High }) {
                true -> Pulse.Low
                false -> Pulse.High
            }
            return receivers.map { Packet(it, name, outPulse) }
        }
    }

    private fun parseUnits(input: String): Map<String,Unit> {
        val types = mutableMapOf<String, UnitType>()
        val nameToReceivers = mutableMapOf<String, List<String>>()

        input
            .lines()
            .forEach { line ->
                val (sender, receiversString) = line.split(" -> ")
                val (type, name) = when(sender.first()) {
                    '&' -> UnitType.Conjunction to sender.drop(1)
                    '%' -> UnitType.FlipFlop to sender.drop(1)
                    else -> UnitType.BroadCaster to sender
                }
                types[name] = type
                nameToReceivers[name] = receiversString.split(", ")
            }

        val senders = mutableMapOf<String, MutableList<String>>()

        for ((name, receivers) in nameToReceivers) {
            for (rec in receivers) {
                senders.getOrPut(rec) { mutableListOf() }.add(name)
            }
        }

        return types.map { (name, type) ->
            val rs = nameToReceivers.getValue(name)
            val unit = when (type) {
                UnitType.BroadCaster -> Broadcaster(name, receivers = rs)
                UnitType.FlipFlop -> FlipFlop(name, receivers = rs, turnedOn = false)
                UnitType.Conjunction -> Conjunction(name, receivers = rs, senders = senders.getValue(name))
            }
            name to unit
        }
        .toMap()
    }
}