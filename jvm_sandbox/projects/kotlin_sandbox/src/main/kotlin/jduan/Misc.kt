package jduan.misc

data class Person(val name: String,
    // null by default
                  val age: Int? = null)

data class Person2(var name: String,
                   var isMarried: Boolean)

fun printPerson2(person: Person2) {
    // in java: person.setName("John")
    person.name = "John"
    // in java: person.getName()
    println(person.name)
    // in java: person.setMarried(false)
    person.isMarried = false
    // in java: person.isMarried()
    println(person.isMarried)
}

class Rectangle(val height: Int, val width: Int) {
    val isSquared: Boolean
        get() {
            return height == width
        }
}

// This shows that "property get" isn't cached!
class Config {
    var flag: Boolean
        get() {
            println("calling get")
            return true
        }
        set(value: Boolean) {
            println("calling set")
        }
}

fun longestCommonPath(path1: String, path2: String): String {
    val parts1 = path1.split("/")
    val parts2 = path2.split("/")
    var idx = 0
    while (idx < Integer.min(parts1.size, parts2.size)) {
        if (parts1[idx] != parts2[idx]) {
            break
        }
        idx++
    }
    return parts1.subList(0, idx).joinToString(separator = "/")
}

fun main() {
    val task = com.sun.tools.jdeps.Analyzer()
    val path1 = "projects/search/common/serviceapi/src/main/java/com/airbnb/search/common/serviceapi/request"
    val path2 = "projects/search/common/serviceapi/src/main/java/com/airbnb/search/common/serviceapi/response"
    println("longestCommonPath: ${longestCommonPath(path1, path2)}")
    println("hello")
    val config = Config()
    repeat(5) {
        config.flag
    }
}
