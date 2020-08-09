package jduan

fun main() {
    val map = mapOf(
            "hello" to "world",
            "name" to listOf("john", "david")
    )
    println("hello -> ${map["hello"]}")

    // as? returns null if the conversation fails
    val name = map["bad key"] as? List<String>
    // as throws an exception if the conversation fails
    val name2 = map["name"] as List<String>
    println(name)
    println(name2)
}
