package jduan

import io.reactivex.Flowable
import io.reactivex.Observable
import java.io.IOException



fun hello(vararg args: String) {
  Observable.fromArray(args).subscribe(
    {s -> println("Hello $s!")},
    {s -> println("Hello $s!")}
  )
}

fun helloError() {
  val error = Observable.error<String>(IOException())

  error.subscribe(
    { v -> println("This should never be printed!") },
    { e -> throw e },
    { println("This neither!") })
}

fun main(args: Array<String>) {
  hello("Ben", "George")
  helloError()
}
