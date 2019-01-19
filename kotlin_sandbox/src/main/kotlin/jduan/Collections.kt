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