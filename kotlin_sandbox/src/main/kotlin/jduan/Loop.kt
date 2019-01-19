package jduan.loop

import java.util.*

fun fizzBuzz(i: Int) =
        when {
            i % 15 == 0 -> "FizzBuzz"
            i % 3 == 0 -> "Fizz"
            i % 5 == 0 -> "Buzz"
            else -> "$i"
        }

fun testLoops() {
    // ranges are right inclusive
    for (i in 1..100) {
        print("${fizzBuzz(i)} ")
    }
    println()
    // use "until" if you don't want "right inclusive"
    // for (x in 0 until size)
    for (i in 100 downTo 1 step 2) {
        print("${fizzBuzz(i)} ")
    }
    println()
}

fun iterateMap() {
    val binaryReps = TreeMap<Char, String>()

    for (c in 'A' .. 'F') {
        val binary = Integer.toBinaryString(c.toInt())
        binaryReps[c] = binary
    }

    for ((char, binary) in binaryReps) {
        println("$char = $binary")
    }
}

fun iterateList() {
    val list = arrayListOf("10", "11", "1001")
    for ((index, element) in list.withIndex()) {
        println("$index: $element")
    }
}

fun isLetter(c: Char) = c in 'a'..'z' || c in 'A'..'Z'

fun isNotDigit(c: Char) = c !in '0'..'9'

fun recognize(c: Char) = when (c) {
    in '0'..'9' -> "It's a digit!"
    in 'a'..'z', in 'A'..'Z' -> "It's a letter!"
    else -> "I don't know..."
}

