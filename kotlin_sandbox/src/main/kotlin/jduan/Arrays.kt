package jduan

import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("")

fun main() {
    val nums = arrayOf(1, 2, 3)
    nums.forEach { println(it) }

    val nulls = arrayOfNulls<String>(5)
    nulls.forEach { println(it) }

    val asc = Array(5) {i -> (i * i).toString()}
    asc.forEach { println(it) }

    // primitive type arrays: ByteArray, ShortArray, IntArray, etc
    val x: IntArray = intArrayOf(1, 2, 3)
    x[0] = x[1] + x[2]
    x.forEach { println(it) }

    val list = arrayListOf("10", "11", "1001")
    list.withIndex().forEach { (idx, str) ->
        println("$idx: $str")
    }
}
