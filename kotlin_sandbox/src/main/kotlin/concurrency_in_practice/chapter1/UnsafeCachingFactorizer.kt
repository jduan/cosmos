package concurrency_in_practice.chapter1

import java.util.concurrent.atomic.AtomicReference
import javax.annotation.concurrent.NotThreadSafe
import javax.annotation.concurrent.ThreadSafe
import kotlin.math.sqrt

/**
 * The morale of the story is: to preserve state consistency, update related state
 * variables in a single atomic operation.
 *
 * Here's what happens:

(22:42:10.164) Thread Thread-0: getting factors for 100
(22:42:10.206) Thread Thread-0: set last number to 100
(22:42:10.638) Thread Thread-1: getting factors for 200
(22:42:10.639) Thread Thread-1: set last number to 200
(22:42:10.639) Thread Thread-1: set last factors to [2, 2, 2, 5, 5]
(22:42:11.209) Thread Thread-0: set last factors to [2, 2, 5, 5]
(22:42:11.209) Thread main: getting factors for 200
(22:42:11.209) Thread main: last number is 200, return early
(22:42:11.209) Thread main: wrong factors!

 */
fun main() {
  val ucf = UnsafeCachingFactorizer()
  val n1 = 100
  val n2 = 200
  val thread1 = Thread(Runnable { ucf.getFactors(n1) } )
  val thread2 = Thread(Runnable { ucf.getFactors(n2) } )
  thread1.start()
  Thread.sleep(500)
  thread2.start()
  thread1.join()
  thread2.join()
  val factors = ucf.getFactors(n2)
  val product = factors.reduce { acc, i -> acc * i }
  if (product != n2) {
    Utils.println("wrong factors!")
  }
}

@NotThreadSafe
class UnsafeCachingFactorizer {
  private val lastNumber = AtomicReference<Int>()
  private val lastFactors = AtomicReference<List<Int>>()

  fun getFactors(n: Int): List<Int> {
    Utils.println("getting factors for $n")
    return if (n == lastNumber.get()) {
      Utils.println("last number is $n, return early")
      lastFactors.get()
    } else {
      val factors = GFG.primeFactors(n)
      Utils.println("set last number to $n")
      lastNumber.set(n)
      if (n == 100) {
        Thread.sleep(1000)
      }
      Utils.println("set last factors to $factors")
      lastFactors.set(factors)
      factors
    }
  }
}

@ThreadSafe
class SafeCachingFactorizer {
  private val lastNumber = AtomicReference<Int>()
  private val lastFactors = AtomicReference<List<Int>>()

  fun getFactors(n: Int): List<Int> {
    Utils.println("getting factors for $n")
    return if (n == lastNumber.get()) {
      Utils.println("last number is $n, return early")
      lastFactors.get()
    } else {
      val factors = GFG.primeFactors(n)
      synchronized(this) {
        Utils.println("set last number to $n")
        lastNumber.set(n)
        if (n == 100) {
          Thread.sleep(1000)
        }
        Utils.println("set last factors to $factors")
        lastFactors.set(factors)
      }
      factors
    }
  }
}

internal object GFG {
  // A function to print all prime factors
  // of a given number n
  fun primeFactors(n: Int): List<Int> {
    var n = n
    val factors = mutableListOf<Int>()
    // Print the number of 2s that divide n
    while (n % 2 == 0) {
      factors.add(2)
      n /= 2
    }

    // n must be odd at this point.  So we can
    // skip one element (Note i = i +2)
    var i = 3
    while (i <= sqrt(n.toDouble())) {
      // While i divides n, print i and divide n
      while (n % i == 0) {
        factors.add(i)
        n /= i
      }
      i += 2
    }

    // This condition is to handle the case when
    // n is a prime number greater than 2
    if (n > 2)
      factors.add(n)

    return factors
  }
}

