package concurrency_in_practice

import java.time.LocalTime

object Utils {
    // Prefix with current time and thread name
    fun println(str: String) {
        println("(${LocalTime.now()}) Thread ${Thread.currentThread().name}: $str")
    }
}
