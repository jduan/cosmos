package jduan

fun main() {
  val startTime = System.nanoTime()
  Thread.sleep(2000)
  val duration = (System.nanoTime() - startTime) / 1_000_000_000
  println("duration: $duration")
}
