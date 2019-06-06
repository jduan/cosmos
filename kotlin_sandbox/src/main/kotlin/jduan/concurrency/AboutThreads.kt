package jduan.concurrency

fun main() {
    val messages = arrayOf(
        "Mares eat oats",
        "Does eat oats",
        "Little lambs eat ivy",
        "A kid will eat ivy too"
    )

    messages.forEach {
        Thread.sleep(1000)
        println(it)
    }
}
