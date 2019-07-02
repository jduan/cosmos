package concurrency_in_practice.chapter4

import java.util.concurrent.CopyOnWriteArrayList

/**
 * CopyOnWriteArrayList is a thread-safe variant of ArrayList in which all mutative operations
 * (add, set, and so on) are implemented by making a fresh copy of the underlying array.
 * This is ordinarily too costly, but may be more efficient than alternatives when traversal
 * operations vastly outnumber mutations, and is useful when you cannot or don't want to
 * synchronize traversals, yet need to preclude interference among concurrent threads.
 *
 * There's also CopyOnWriteArraySet.
 */
fun main() {
//    val numbers = mutableListOf(1, 2, 3, 4)
    val numbers = CopyOnWriteArrayList(listOf(1, 2, 3, 4))
    println("numbers hashcode: ${numbers.hashCode()}")
    for (number in numbers) {
        // It's ok to add or remove elements while iterating
        if (number == 2) {
            numbers.add(10)
        }
        if (number == 4) {
            numbers.removeAt(4)
        }
        println(number)
    }

    println("numbers: $numbers")
    println("numbers hashcode: ${numbers.hashCode()}")
}
