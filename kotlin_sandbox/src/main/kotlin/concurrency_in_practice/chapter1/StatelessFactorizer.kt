package concurrency_in_practice.chapter1

import java.math.BigInteger
import java.util.concurrent.atomic.AtomicLong
import javax.annotation.concurrent.NotThreadSafe
import javax.annotation.concurrent.ThreadSafe

@ThreadSafe
// This class is thread safe because it's stateless. The getFactors function
// only takes some input and returns some output.
class StatelessFactorizer {
  fun getFactors(i: BigInteger): List<BigInteger> {
    // mimic long computation by sleeping
    Thread.sleep(1000)
    return listOf(BigInteger.ONE)
  }
}

@NotThreadSafe
class UnsafeCountingFactorizer {
  private var count: Long = 0

  fun getCount() = count

  fun service() {
    // sleep a bit to simulate "long computation"
    Thread.sleep(1000)
    // ++ is why this class isn't thread safe because ++ is 3 operations
    // read, modify, and update
    count++
  }
}

@ThreadSafe
// Use an AtomicLong to make it thread safe.
class SafeCountingFactorizer {
  private var count = AtomicLong(0)

  fun getCount() = count

  fun service() {
    // sleep a bit to simulate "long computation"
    Thread.sleep(1000)
    // ++ is why this class isn't thread safe because ++ is 3 operations
    // read, modify, and update
    count.incrementAndGet()
  }
}

fun test1() {
  val ucf = UnsafeCountingFactorizer()
  val threads = (1..10).map {
    Thread(Runnable {
      ucf.service()
    })
  }
  threads.forEach { it.start() }
  threads.forEach { it.join() }
  // The final count should be 10 if the class is thread safe
  // You will most likely get a number smaller than 10!
  println("final count: ${ucf.getCount()}")
}

fun test2() {
  val scf = SafeCountingFactorizer()
  val threads = (1..10).map {
    Thread(Runnable {
      scf.service()
    })
  }
  threads.forEach { it.start() }
  threads.forEach { it.join() }
  // The final count should always be 10 because the class is thread safe.
  println("final count: ${scf.getCount()}")
}

fun main() {
  test2()
}
