import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.async
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.util.concurrent.atomic.AtomicLong

fun main() {
//    simpleCoroutine()
//    lotsOfCoroutines()
    lotsOfCoroutines2()
}

fun lotsOfCoroutines() {
    val c = AtomicLong()
    // Launch 1 million coroutines and increment the atomic long.
    for (i in 1..1_000_000L) {
        GlobalScope.launch {
            c.addAndGet(i)
        }
    }

    // 500000500000
    println("Total: ${c.get()}")
}

fun lotsOfCoroutines2() {
    val deferred = (1..1_000_000).map {n ->
        GlobalScope.async {
            // We delay here in order to show that coroutines run in paralle. Otherwise, if they
            // run sequentially, this would take 1 million seconds to run.
            delay(1000)
            n
        }
    }

    runBlocking {
        val sum = deferred.map {
            it.await().toLong()
        }.sum()
        println("sum: $sum")
    }
}

fun simpleCoroutine() {
    println("Start")
    // this launches a coroutine and schedules it to run in the GlobalScope
    GlobalScope.launch {
        // delay is similar to sleep but it doesn't block a thread, it only suspends the coroutine
        // itself. The thread is returned to the pool while the coroutine is waiting, and when the
        // waiting is done, the coroutine resumes on a free thread in the pool.
        delay(1000)
        println("Hello")
    }

//    Thread.sleep(2000)
    // Start a coroutine and waits until it's done.
    runBlocking {
        delay(2000)
    }
    println("Stop")
}
