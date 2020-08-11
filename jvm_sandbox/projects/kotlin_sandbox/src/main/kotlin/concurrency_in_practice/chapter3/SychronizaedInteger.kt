package concurrency_in_practice.chapter3

import org.slf4j.LoggerFactory
import javax.annotation.concurrent.GuardedBy
import javax.annotation.concurrent.NotThreadSafe
import javax.annotation.concurrent.ThreadSafe

private val logger = LoggerFactory.getLogger("main")

/**
 * Locking is not just about mutual exclusion; it is also about memory visibility.
 * To ensure that all threads see the most up-to-date values of shared mutable variables,
 * the reading and writing threads must synchronize on a common lock.
 */

@NotThreadSafe
class MutableInteger {
    private var value: Int = 0

    // Without synchronization, a reader thread can see "stale data"
    // that was set by another writer thread.
    fun get(): Int = value

    fun set(value: Int) {
        this.value = value
    }
}

// Synchronized makes sure data doesn't get stale.
@ThreadSafe
class SynchronizedInteger {
    @GuardedBy("this")
    private var value: Int = 0

    @Synchronized
    fun get(): Int = value

    @Synchronized
    fun set(value: Int) {
        this.value = value
    }
}

fun main() {
    val wrappedInteger = SynchronizedInteger()

    val thread1 = Thread(Runnable {
        Thread.sleep(1000)
        wrappedInteger.set(10)
    })

    val thread2 = Thread(Runnable {
        logger.info("wrappedInteger is ${wrappedInteger.get()}")
    })

    thread2.start()
    thread1.start()
    thread1.join()
    thread2.join()

    logger.info("done")
}
