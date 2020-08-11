package jduan

import com.google.common.math.Stats

// Calculate population variance of a set of numbers
fun populationVariance() {
    val values = mutableListOf<Int>()
    (1..300).forEach {
        values.add(1)
    }
//    val indices = listOf(8, 20, 27, 100, 120, 121, 140)
//    val indices = listOf(8, 20, 27, 100)
    val indices = listOf(8)
    indices.forEach {
        values[it] = 0
    }
    println("values: $values")
    println("size: ${values.size}")

    val variance = Stats.of(values).populationVariance()
    val multiplier = 4
    println("variance: ${variance * multiplier}")
}

fun main() {
    populationVariance()
}
