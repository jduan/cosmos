package concurrency_in_practice.chapter3

import org.slf4j.LoggerFactory
import java.util.concurrent.atomic.AtomicInteger

private val logger = LoggerFactory.getLogger("")

class ConnectionId {
    companion object {
        private val nextId = AtomicInteger(0)
        // create an object of an anonymous inner class that subclasses ThreadLocal
        // to override the "initialValue" method, which defaults to return null.
        private val connectionId = object : ThreadLocal<Int>() {
            override fun initialValue(): Int {
                return nextId.getAndIncrement()
            }
        }

        fun get(): Int {
            // calling remove() would force the ThreadLocal to call initialValue() again
            connectionId.remove()
            return connectionId.get()
        }
    }
}

fun main() {
    val thread = Thread(Runnable {
        repeat(5) {
            logger.info("My connection id is: ${ConnectionId.get()}")
        }
    })
    thread.start()
    thread.join()

    logger.info("Done!")
}
