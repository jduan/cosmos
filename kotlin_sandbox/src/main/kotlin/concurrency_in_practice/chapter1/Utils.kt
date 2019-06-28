package concurrency_in_practice.chapter1

import java.time.LocalTime

object Utils {
  fun println(str: String) {
    kotlin.io.println("(${LocalTime.now()}) Thread ${Thread.currentThread().name}: $str")
  }
}
