package concurrency_in_practice.chapter1

import java.math.BigInteger
import javax.annotation.concurrent.ThreadSafe

@ThreadSafe
class StatelessFactorizer {
  fun getFactors(i: BigInteger): List<BigInteger> {
    // mimic long computation by sleeping
    Thread.sleep(1000)
    return listOf(BigInteger.ONE)
  }
}

fun main() {
  println(StatelessFactorizer().getFactors(BigInteger.valueOf(3)))
}
