package jduan

import junit.framework.TestCase.assertEquals
import org.junit.Test

class HelloWorldTest {
    // This test fails on purpose, in order to test the "excludeTest" thing in build.gradle
    @Test
    fun `hello world`() {
        assertEquals("Hello, World!", "Hello, world!")
    }
}
