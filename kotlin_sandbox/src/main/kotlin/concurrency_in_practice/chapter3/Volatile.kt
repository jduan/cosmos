package concurrency_in_practice.chapter3

import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("")

/**
 * "volatile" variables are not cached in registers or in CPU caches where they
 * are hidden from other processors, so a read of a volatile variable always
 * returns the most recent write by any thread.
 *
 * Note that locking can guarantee both visibility and atomicity; volatile variables
 * can only guarantee visibility.
 */
class Flag {
    @Volatile
    var asleep = false
}

fun main() {
    val flag = Flag()

    val thread1 = Thread(Runnable {
        while (!flag.asleep) {
            logger.info("I'm still not asleep yet.")
            Thread.sleep(1000)
        }

        logger.info("I'm going to sleep now!")
    })

    val thread2 = Thread(Runnable {
        Thread.sleep(5000)
        logger.info("Setting 'asleep' to true.")
        flag.asleep = true
    })

    thread1.start()
    thread2.start()
    thread1.join()
    thread2.join()

    logger.info("Done!")
}
