package concurrency_in_practice.chapter1

import javax.annotation.concurrent.NotThreadSafe
import javax.annotation.concurrent.ThreadSafe

class MyString {}

@NotThreadSafe
class LazyInitRace {
  companion object {
    private var instance: MyString? = null
    fun getInstance(): MyString {
      if (instance == null) {
        // This simulates it takes a long time to create an object!
        Thread.sleep(1000)
        instance = MyString()
      }

      return instance!!
    }
  }
}

@ThreadSafe
class LazyInitRace2 {
  companion object {
    private var instance: MyString? = null
    // @Synchronized is the counterpart of Java's synchronized keyword
    @Synchronized fun getInstance(): MyString {
      if (instance == null) {
        // This simulates it takes a long time to create an object!
        Thread.sleep(1000)
        instance = MyString()
      }

      return instance!!
    }
  }
}

fun main() {
  val threads = (1..10).map {
    Thread(Runnable {
//      println("instance hash code: ${LazyInitRace.getInstance().hashCode()}")
      println("instance hash code: ${LazyInitRace2.getInstance().hashCode()}")
    })
  }
  threads.forEach { it.start() }
  threads.forEach { it.join() }
}
