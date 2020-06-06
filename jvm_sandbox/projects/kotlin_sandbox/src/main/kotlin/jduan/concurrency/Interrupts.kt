package jduan.concurrency

fun main() {
    val t1 = Thread(Runnable {
        val messages = arrayOf(
            "Mares eat oats",
            "Does eat oats",
            "Little lambs eat ivy",
            "A kid will eat ivy too"
        )

        for (i in 0 until messages.size) {
            println(messages[i])
            try {
                Thread.sleep(1000)
            } catch (ex: InterruptedException) {
                break
            }
        }
    })
    t1.start()

    // Sleep, then interrupt t1
    Thread.sleep(2000)
    t1.interrupt()
}
