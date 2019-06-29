package jduan.coroutines

import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

fun testLaunch() {
    GlobalScope.launch {
        delay(1000L)
        println("world!")
        GlobalScope.launch {
            while (true) {
                delay(1000L)
                println("I'm still here")
            }
        }
    }
    println("hello,")
    Thread.sleep(20000L)
}

fun main() {
    testLaunch()
}
