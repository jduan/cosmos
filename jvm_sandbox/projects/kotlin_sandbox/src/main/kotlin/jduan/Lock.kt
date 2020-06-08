package jduan

import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch

fun main() {
    val lock = Object()

    // there's no deadlock, which shows locks in Java are re-entrant
    synchronized(lock) {
        println("I'm in the synchronized block")
        synchronized(lock) {
            println("I've acquired the lock again")
        }
    }

    println("done")

    val gitRepo = GitRepo()
    val job1 = GlobalScope.launch {
        gitRepo.read()
    }
    val job2 = GlobalScope.launch {
        gitRepo.read()
    }

    Thread.sleep(10_000)
    println("Main thread is done")
}

class GitRepo {
    private val lock = Object()
    fun read() {
        synchronized(lock) {
            println("Lock acquired")
            println("thread: ${Thread.currentThread().id}, name: ${Thread.currentThread().name}")
            Thread.sleep(3000)
            println("Lock released")
        }
    }
}
