package year2018.day4

import java.io.File

data class Sleep(
    // The minute the sleep starts
    val start: Int,
    // The minute the sleep ends
    val end: Int
) {
  fun minutes() = end - start
}

data class GuardSleep(
    val guardId: Int
) {
  private val sleeps = mutableListOf<Sleep>()

  fun addSleep(sleep: Sleep): GuardSleep {
    sleeps.add(sleep)
    return this
  }

  fun totalSleepMinutes() =
      sleeps.fold(0) { acc, sleep ->  acc + sleep.minutes()}

  // Return the most slept minute and how many times
  fun sleepMostMinute(): Pair<Int, Int> {
    val minuteCount = mutableMapOf<Int, Int>()
    sleeps.forEach {sleep ->
      for (minute in sleep.start until sleep.end) {
        val count = minuteCount.getOrDefault(minute, 0)
        minuteCount[minute] = count + 1
      }
    }
    val maxCount = minuteCount.values.max()
    val counts = minuteCount.filter { entry -> entry.value == maxCount }
    return counts.keys.first() to maxCount!!
  }

  override fun toString(): String {
    return "GuardSleep(guardId=${guardId}, sleeps=${sleeps}"
  }
}

sealed class Event(open val hour: Int, open val minute: Int)

data class ShiftStarts(
    val guardId: Int,
    override val hour: Int,
    override val minute: Int
) : Event(hour, minute)

data class FallAsleep(
    override val hour: Int,
    override val minute: Int
) : Event(hour, minute)

data class WakesUp(
    override val hour: Int,
    override val minute: Int
) : Event(hour, minute)

fun parseLine(line: String): Event {
  val shiftStartsRegex = """\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] Guard #(\d+) begins shift""".toRegex()
  val fallAsleepRegex = """\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] falls asleep""".toRegex()
  val wakesUpRegex = """\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] wakes up""".toRegex()
  return when {
    shiftStartsRegex.matches(line) -> {
      val (_, hour, minute, guardId) = shiftStartsRegex.find(line)!!.groupValues
      ShiftStarts(guardId.toInt(), hour.toInt(), minute.toInt())
    }
    fallAsleepRegex.matches(line) -> {
      val (_, hour, minute) = fallAsleepRegex.find(line)!!.groupValues
      FallAsleep(hour.toInt(), minute.toInt())
    }
    wakesUpRegex.matches(line) -> {
      val (_, hour, minute) = wakesUpRegex.find(line)!!.groupValues
      WakesUp(hour.toInt(), minute.toInt())
    }
    else -> throw Exception("Unexpected line: $line")
  }
}

fun parseFile(path: String): Map<Int, GuardSleep> {
  val lines = File(path).readLines().sorted()
  val guardToSleep = mutableMapOf<Int, GuardSleep>()
  var guardId: Int = 0
  var sleepStart: Int = 0
  lines.forEach { line ->
    val event = parseLine(line)
    when(event) {
      is ShiftStarts -> {
        guardId = event.guardId
      }
      is FallAsleep -> {
        sleepStart = event.minute
      }
      is WakesUp -> {
        val sleepEnd = event.minute
        val guardSleep = guardToSleep[guardId]
        if (guardSleep == null) {
          val gs = GuardSleep(guardId)
          gs.addSleep(Sleep(sleepStart, sleepEnd))
          guardToSleep[guardId] = gs
        } else {
          guardSleep.addSleep(Sleep(sleepStart, sleepEnd))
        }
      }
    }
  }

  return guardToSleep
}

// Find the guard that spent the most minutes asleep.

// Find the minute the guard that was asleep the most.
