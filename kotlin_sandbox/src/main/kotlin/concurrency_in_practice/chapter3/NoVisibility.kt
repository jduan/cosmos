package concurrency_in_practice.chapter3

import org.slf4j.LoggerFactory

val logger = LoggerFactory.getLogger("main")

fun main() {
    logger.info("Entering main")
    var ready = false
    var number = 0

    // It's possible that this thread never terminates because "ready" may never become
    // visible to this thread without synchronization!
    val thread = Thread(Runnable {
        var count = 0
        while (!ready) {
//            logger.info("yielding")
            count++
            Thread.yield()
        }
        logger.info("Count is $count")
        // It's possible that this prints 0 because "number" may never become visible
        // to this thread without synchronization!
        logger.info("Number is $number")
    })
    thread.start()

    Thread.sleep(1000)
    number = 42
    ready = true
}
