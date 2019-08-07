package concurrency_in_practice.chapter4

import java.util.concurrent.atomic.AtomicInteger
import javax.annotation.concurrent.NotThreadSafe

/**
 * This class isn't thread-safe because the setLower and setUpper methods are
 * "check-then-act" sequences, but they don't use sufficient locking to make
 * them atomic.
 *
 * While the underlying AtomicIntegers are thread-safe, the composite class isn't.
 * Because the underlying state variables "lower" and "upper" are not independent.
 * NumberRange can't simply delegate thread safety to its thread-safe state variables!
 */
@NotThreadSafe
class NumberRange {
    private val lower = AtomicInteger(0)
    private val upper = AtomicInteger(100)

    fun setLower(i: Int) {
        if (i > upper.get()) {
            throw IllegalArgumentException("Can't set lower to $i > $upper")
        }
        // make thread to context switch
        Thread.sleep(100)
        lower.set(i)
    }

    fun setUpper(i: Int) {
        if (i < lower.get()) {
            throw IllegalArgumentException("Can't set upper to $i < $lower")
        }
        upper.set(i)
    }

    fun isInRange(i: Int) = i >= lower.get() && i <= upper.get()

    override fun toString(): String = "NumberRange(${lower.get()}, ${upper.get()})"
}

fun main() {
    val range = NumberRange()
    println("range: $range")
    val thread1 = Thread(Runnable {
        range.setLower(90)
    }).also { t -> t.start() }
    val thread2 = Thread(Runnable {
        range.setUpper(50)
    }).also { t -> t.start() }
    thread1.join()
    thread2.join()
    println("range: $range")
}
