package jduan.collections

fun <T> Collection<T>.joinToString(
    separator: String = ", ",
    prefix: String = "(",
    suffix: String = ")"
): String {
    val result = StringBuilder(prefix)
    for ((index, element) in this.withIndex()) {
        if (index > 0) result.append(separator)
        result.append(element)
    }

    result.append(suffix)
    return result.toString()
}

// extension property

var StringBuilder.lastChar: Char
    get() = get(length - 1)
    set(value: Char) {
        this.setCharAt(length - 1, value)
    }

private data class Person(val name: String)

fun main() {
    val set = hashSetOf(1, 7, 53)
    println("max element of set: ${set.max()}")

    val people = setOf(Person("John"), Person("David"))
    println("max element of people set: ${people.maxBy(Person::name)}")

    println("people: ${people.joinToString(", ", prefix = "[", suffix = "]")}")
}
