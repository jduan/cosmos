package jduan

fun main() {
  val lock = Object()

  // there's no deadlock
  synchronized(lock) {
    println("I'm in the synchronized block")
    synchronized(lock) {
      println("I've acquired the lock again")
    }
  }

  println("done")
}
