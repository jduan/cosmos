package jduan.concurrency

import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

fun main() {
    interrupt3()
}

fun interrupt1() {
    val thread = Thread(Runnable {
        for (i in 0 until 10) {
            println(i)
            Thread.sleep(1_000)
        }
    })
    thread.start()
    println("interrupt task thread")
    Thread.sleep(3_000)
    thread.interrupt()
    println("wait for 1 second")
    thread.join(1_000)
    println("Main thread shut down!")
}

fun interrupt2() {
    val executor = Executors.newFixedThreadPool(10)
    val future = executor.submit(Runnable {
        for (i in 0 until 10) {
            println(i)
            Thread.sleep(1_000)
        }
    })

    println("sleep for 3 seconds")
    Thread.sleep(3_000)
    println("cancel task")
    // the behavior would be different it you change the param to false!
    future.cancel(true)
    println("wait for 1 second")
    Thread.sleep(1_000)
    println("shut down executor")
    executor.shutdown()
    println("Main thread shut down!")
}

fun interrupt3() {
    val executor = Executors.newSingleThreadExecutor()
    executor.submit {
        printNumbers()
        printNumbers()
    }

    Thread.sleep(3000)
    executor.shutdownNow()
    executor.awaitTermination(3, TimeUnit.SECONDS)
}

fun printNumbers() {
    for (i in 0 until 10) {
        println(i)
        try {
            Thread.sleep(1_000)
        } catch (ex: InterruptedException) {
            // If you don't preserve the "interruption status", the 2nd call
            // of printNumbers() would run fully for 9 seconds.
            Thread.currentThread().interrupt()
            break
        }
    }
}

