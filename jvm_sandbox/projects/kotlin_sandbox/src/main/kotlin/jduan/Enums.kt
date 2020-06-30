package jduan

import org.slf4j.LoggerFactory
import java.lang.Exception

private val logger = LoggerFactory.getLogger("")

enum class Color {
    RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET;
    override fun toString(): String =
        when(this) {
            Color.RED -> "I'm red"
            Color.ORANGE -> "orange"
            Color.YELLOW -> "yellow"
            Color.GREEN -> "green"
            Color.BLUE -> "blue"
            Color.INDIGO -> "indigo"
            Color.VIOLET -> "violet"
        }
}

fun getColorName(color: Color): String =
    // "when" expressions are exhaustive. If you remove one of the colors,
    // you will get a compiler error.
    when(color) {
        Color.RED -> "red"
        Color.ORANGE -> "orange"
        Color.YELLOW -> "yellow"
        Color.GREEN -> "green"
        Color.BLUE -> "blue"
        Color.INDIGO -> "indigo"
        Color.VIOLET -> "violet"
    }

fun mixColors(color1: Color, color2: Color): Color =
    when(setOf(color1, color2)) {
        setOf(Color.RED, Color.YELLOW) -> Color.ORANGE
        setOf(Color.YELLOW, Color.BLUE) -> Color.GREEN
        setOf(Color.BLUE, Color.VIOLET) -> Color.INDIGO
        else -> throw Exception("Dirty color")
    }

// By default, enums are sorted by their ordinal.
fun sortColors(colors: List<Color>) {
    println("color sorted: ${colors.sorted()}")
}

fun main() {
    logger.info(getColorName(Color.RED))
    logger.info(getColorName(Color.RED))
    logger.info(mixColors(Color.RED, Color.YELLOW).toString())
    sortColors(listOf(Color.RED, Color.VIOLET, Color.BLUE, Color.YELLOW))
    println("Red is ${Color.RED}")
}
