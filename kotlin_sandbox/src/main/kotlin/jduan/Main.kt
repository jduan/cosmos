package jduan

import java.io.BufferedReader

fun main() {
    val tests = listOf(
        "hello",
        "world"
    )
    val str = """
        You're being notified because it seems like you are the owner of one or more tests
        that have been disabled. They aren't being run on CI because of that.
        You can see more info at http://arborist-web.d.musta.ch/find_tests.
        Please fix them at your earliest convenience and re-enable them. See
        https://air/unit-test-policy instructions. If you have any questions,
        feel free to reach out to #build-infra on Slack. Here's the list of tests:
        $tests
        """.trimIndent()
    println(str)

    println("Result is: ${hello()}")

    parseNumber("33")
}

// This shows that "return" still works even if you have a "finally" block
fun hello(): Int {
    try {
        println("hello")
        return 1
    } finally {
        println("world")
    }
}

fun parseNumber(maybeNumber: String) {
    val number = try {
        Integer.parseInt(maybeNumber)
    } catch (e: NumberFormatException) {
        return
    }

    println(number)
}
