package concurrency_in_practice.chapter1

import java.math.BigInteger
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
    Thread.sleep(3000)
    // ++ is why this class isn't thread safe because ++ is 3 operations
    // read, modify, and update
    count++
  }
}

fun main() {
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
