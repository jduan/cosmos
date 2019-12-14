package jduan

import java.lang.Thread.sleep
import kotlin.concurrent.thread

fun main() {
    val t1 = thread {
        println("going to sleep now")
        sleep(1000)
        println("I'm awake now")
    }

    t1.join()
    // This shows you can call "join" on a thread many times.
    t1.join()

    println("main: done")
}
