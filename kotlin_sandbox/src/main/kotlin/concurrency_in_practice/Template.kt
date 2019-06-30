package concurrency_in_practice

import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("")

fun main() {
    val thread = Thread(Runnable {
        logger.info("Hello")
        Thread.sleep(1000)
    })
    thread.start()
    thread.join()

    logger.info("Done!")
}
