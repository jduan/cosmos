package jduan.main

import java.io.File

fun main(args: Array<String>) {
    val file = File("/tmp/100")
    file.bufferedReader().use {
        println(it.readText())
    }

    println("bye!")
}
