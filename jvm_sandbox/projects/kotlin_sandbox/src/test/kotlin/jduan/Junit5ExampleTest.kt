package jduan

import org.junit.Ignore
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.MethodOrderer
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestMethodOrder
import org.junit.jupiter.migrationsupport.EnableJUnit4MigrationSupport

//@TestMethodOrder(MethodOrderer.Alphanumeric::class)
@TestMethodOrder(MethodOrderer.Random::class)
// See https://junit.org/junit5/docs/snapshot/user-guide/#writing-tests-parallel-execution
// You can configure parallelism at the test class level as well as the test level.
// You get 4 combinations:
// * execute classes sequentially and their methods sequentially
// * execute classes sequentially but their methods concurrently
// * execute classes concurrently but their methods sequentially
// * execute classes concurrently and their methods concurrently
@EnableJUnit4MigrationSupport
//! We are doing "automatic extension registration", see
// https://junit.org/junit5/docs/current/user-guide/#extensions-registration-automatic
// @ExtendWith(TimingExtension::class)
class Junit5ExampleTest {
    @Test
    @Ignore
    fun testZ() {
        println("running testZ")
        assertEquals(2, 1 + 1)
        Thread.sleep(1000)
        println("done running testZ")
    }

    @Test
    fun testA() {
        println("thread id in testA: ${Thread.currentThread()}")
        println("running testA")
        assertEquals(2, 1 + 1)
        Thread.sleep(1000)
        println("done running testA")
    }

    @Test
    fun testY() {
        println("running testY")
        assertEquals(2, 1 + 1)
        Thread.sleep(1000)
        println("done running testY")
    }

    @Test
    fun testE() {
        println("running testE")
        assertEquals(2, 1 + 1)
        Thread.sleep(1000)
        println("done running testE")
    }

    @Test
    fun testB() {
        println("running testB")
        assertEquals(2, 1 + 1)
        Thread.sleep(1000)
        println("done running testB")
    }
}
