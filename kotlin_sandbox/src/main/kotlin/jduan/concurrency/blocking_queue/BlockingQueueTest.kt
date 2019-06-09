package jduan.concurrency.blocking_queue

import java.util.concurrent.ArrayBlockingQueue

fun main() {
    val queue = ArrayBlockingQueue<String>(1024)

    val producer = Thread(Runnable {
        try {
            queue.put("1")
            println("sent 1 to the queue")
            Thread.sleep(1000)
            queue.put("2")
            println("sent 2 to the queue")
            Thread.sleep(1000)
            queue.put("3")
            println("sent 3 to the queue")
        } catch (ex: InterruptedException) {
            ex.printStackTrace()
        }
    })

    val consumer = Thread(Runnable {
        try {
            println("got ${queue.take()} from the queue")
            println("got ${queue.take()} from the queue")
            println("got ${queue.take()} from the queue")
        } catch (ex: InterruptedException) {
            ex.printStackTrace()
        }
    })

    producer.start()
    consumer.start()

    producer.join()
    consumer.join()
}
