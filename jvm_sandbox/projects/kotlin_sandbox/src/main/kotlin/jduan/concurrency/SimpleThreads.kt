package jduan.concurrency

fun threadMessage(message: String) {
    val threadName = Thread.currentThread().name
    System.out.printf("%s: %s%n", threadName, message)
}

fun main() {
    val messageLoop = Thread(Runnable {
        val messages = arrayOf(
            "Mares eat oats",
            "Does eat oats",
            "Little lambs eat ivy",
            "A kid will eat ivy too"
        )

        for (i in 0 until messages.size) {
            threadMessage(messages[i])
            try {
                Thread.sleep(4000)
            } catch (ex: InterruptedException) {
                break
            }
        }
    })

//    val patience: Long = 1000 * 60 * 60
    val patience: Long = 1000 * 3

    threadMessage("Starting MessageLoop thread")
    val startTime = System.currentTimeMillis()
    messageLoop.start()

    threadMessage("Waiting for MessageLoop thread to finish")
    while (messageLoop.isAlive) {
        threadMessage("Still waiting...")
        // Wait maximum of 1 second for messageLoop to finish
        messageLoop.join(1000)

        val elapsedTime = System.currentTimeMillis() - startTime
        if (elapsedTime > patience && messageLoop.isAlive) {
            threadMessage("Tired of waiting! Going to interrupt it!")
            messageLoop.interrupt()
            // Shouldn't be long now. Wait indefinitely
            messageLoop.join()
        }
    }

    threadMessage("Finally!")
}
