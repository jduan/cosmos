package concurrency_in_practice.chapter3

import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("")

class Holder(private var n: Int) {
    fun assertSanity() {
        if (n != n) {
            throw AssertionError("This statement is false.")
        } else {
            logger.info("I'm sane.")
        }
    }
}

/**
 * This simulates "unsafe publication of objects". The object "holder" becomes visible
 * to the other thread before it's fully constructed. Hence, the other thread
 * can see it in inconsistent states, which may cause "assertSanity" to throw an exception.
 */
fun main() {
    var holder: Holder? = null
    val thread = Thread(Runnable {
        Thread.sleep(1000)
        logger.info("checking holder")
        holder?.assertSanity()
    })
    holder = Holder(10)

    thread.start()
}
