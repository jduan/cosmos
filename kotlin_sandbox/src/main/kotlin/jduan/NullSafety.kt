package jduan.null_safety

data class Person(val country: Country?)

data class Country(val code: String?)

fun safeCall() {
    val p: Person? = Person(Country("ENG"))
    val res = p?.country?.code
    println("country code: $res")

    val p2: Person? = Person(Country(null))
    val res2 = p2?.country?.code
    // this will print "null"
    println("country code: $res2")
}

fun elvisOperator() {
    var name: String? = null
    var length = name?.length ?: 0
    println("length is $length")

    name = "John"
    length = name?.length ?: 0
    println("length is $length")
}

fun main() {
    safeCall()
    elvisOperator()
}
