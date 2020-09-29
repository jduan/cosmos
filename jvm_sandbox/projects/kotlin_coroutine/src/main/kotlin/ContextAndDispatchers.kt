import kotlinx.coroutines.CoroutineName
import kotlinx.coroutines.launch
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.async
import kotlinx.coroutines.delay
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.slf4j.LoggerFactory

private val log = LoggerFactory.getLogger("ContextAndDispatcher")

fun main() {
//    differentDispatchers()
//    debugCoroutines()
//    jumpBetweenThreads()
//    childrenOfCoroutine()
//    parentalResponsibilities()
//    coroutineName()
    multipleContextElements()
}

private fun differentDispatchers() {
    runBlocking {
        // When launch { ... } is used without parameters, it inherits the context
        // (and thus dispatcher) from the CoroutineScope it is being launched from.
        launch { // context of the parent, main runBlocking coroutine
            println("main runBlocking      : I'm working in thread ${Thread.currentThread().name}")
        }

        // The Unconfined dispatcher is an advanced mechanism that can be useful in certain corner
        // cases but should not be used in general code.
        launch(Dispatchers.Unconfined) { // not confined -- will work with main thread
            println("Unconfined            : I'm working in thread ${Thread.currentThread().name}")
        }

        launch(Dispatchers.Default) { // will get dispatched to DefaultDispatcher
            println("Default               : I'm working in thread ${Thread.currentThread().name}")
        }

        // GlobalScope also uses the DefaultDispatcher
        GlobalScope.launch {
            println("globa scope           : I'm working in thread ${Thread.currentThread().name}")
        }

        // newSingleThreadContext creates a thread for the coroutine to run. A dedicated thread is
        // a very expensive resource.
        launch(newSingleThreadContext("MyOwnThread")) { // will get its own new thread
            println("newSingleThreadContext: I'm working in thread ${Thread.currentThread().name}")
        }
    }
}

// Coroutines can suspend on one thread and resume on another thread.
// Pass the "-Dkotlinx.coroutines.debug" JVM option when running code.
private fun debugCoroutines() {
    // There are 3 coroutines in this code and they all run on the "main" thread.
    runBlocking {
        val a = async {
            log.info("I'm computing a piece of the asnwer")
            5
        }
        val b = async {
            log.info("I'm computing a piece of the asnwer")
            10
        }

        log.info("The asnwer is ${a.await() + b.await()}")
    }
}

private fun jumpBetweenThreads() {
    // "use" releases the thread created with newSingleThreadContext when it's no longer needed
    newSingleThreadContext("Context1").use { ctx1 ->
        newSingleThreadContext("Context2").use { ctx2 ->
            runBlocking(ctx1) {
                // A coroutine's job is part of its context and can be retrieved from it using the
                // "coroutineContext[Job]" expression.
                log.info("My job is ${coroutineContext[Job]}")

                log.info("Started in context1")
                // withContext chnages the context of the current coroutine
                withContext(ctx2) {
                    log.info("Working in context2")
                }
                log.info("Back to context1")
            }
        }
    }
}

// When a coroutine is launched in the CoroutineScope of another coroutine, it inherits its context
// via CoroutineScope.coroutineContext and the Job of the new coroutine becomes a child of the
// parent coroutine's job. When the parent coroutine is cancelled, all its children are recursively
// cancelled, too.
private fun childrenOfCoroutine() {
    runBlocking {
        val request = launch {
            GlobalScope.launch {
                log.info("I'm running in GlobalScope and executing independently") // 1
                delay(1000)
                log.info("I'm not affected by the cancellation of the request") // 4
            }

            // this coroutine inherits the parent contenxt and hence is a child of the parent coroutine
            launch {
                delay(100)
                log.info("I'm a child of the request coroutine") // 2
                delay(1000)
                log.info("I'll not execute this line if my parent is cancelled") // won't show!
            }
        }

        delay(500)
        request.cancel()
        delay(1000)
        log.info("main: who has survived the cancellation?") // 4
    }
}

// A parent coroutine always waits for completion of all its children. A parent does not have to
// explicitly track all the children it launches, and it does not have to use Job.join to wait for
// them at the end.
private fun parentalResponsibilities() {
    runBlocking {
        val request = launch {
            repeat(3) { i ->
                launch {
                    delay((i + 1) * 200L)
                    log.info("Coroutine $i is done")
                }
            }

            log.info("request: I'm done and I don't explicitly join my children that are still alive")
        }

        // A coroutine is only completed once all of its children are completed.
        request.join()
        log.info("Now processing of the request is done")
    }
}

private fun coroutineName() {
    runBlocking {
        val v1 = async(CoroutineName("v1coroutine")) {
            delay(500)
            log.info("Computing v1")
            252
        }
        val v2 = async(CoroutineName("v2coroutine")) {
            delay(1000)
            log.info("Computing v2")
            6
        }

        log.info("The answer for v1/v2 = ${v1.await() / v2.await()}")
    }
}

// Sometimes we need to define multiple elements for a coroutine context. We can use the + operator
// for that.
// See this method in CoroutineContext:
//      public operator fun plus(context: CoroutineContext): CoroutineContext
private fun multipleContextElements() {
    runBlocking {
        launch(Dispatchers.Default + CoroutineName("test")) {
            log.info("I'm working on calcuating PI")
        }
    }
}
