package jduan

import io.reactivex.Flowable

fun hello(vararg args: String) {
  Flowable.fromArray(args).subscribe {s -> println("Hello $s!")}
}

fun main(args: Array<String>) {
  hello("Ben", "George")
}
