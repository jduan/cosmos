package jduan.main

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
}
