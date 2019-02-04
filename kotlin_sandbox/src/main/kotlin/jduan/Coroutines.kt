package jduan.coroutines

import kotlinx.coroutines.*

fun testLaunch() {
    GlobalScope.launch {
        delay(1000L)
        println("world!")
    }
    println("hello,")
    Thread.sleep(2000L)
}
