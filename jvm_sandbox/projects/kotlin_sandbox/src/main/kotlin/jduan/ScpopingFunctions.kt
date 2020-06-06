package jduan.scoping_functions

class Person {
    var name: String? = null
    var age: Int? = null

    fun incrementAge() {
        age = age?.inc()
    }

    override fun toString(): String = "Person(name: $name, age: $age)"
}

data class Book(val title: String, val price: Double)

// "apply" takes a function (which calls "this" and returns nothing),
// applies it, and return this at the end.
/**
inline fun <T> T.apply(block: T.() -> Unit): T {
    block()
    return this
}
*/
fun applyFunction() {
    val peter = Person().apply {
        name = "Peter"
        age = 18
        incrementAge()
    }
    println("person: $peter")
}

// very similar to "apply"
fun alsoFunction() {
    val peter = Person().also {
        it.name = "Peter"
        it.age = 19
    }
    println("person: $peter")
}

// let executes a block of code by passing the object as "it"
// it returns whatever is returned by the block
fun letFunction() {
    val firstName = "Tom"
    val secondName = "Michael"
    val names = listOf(firstName, null, secondName)

    names.forEach { name ->
        name?.let {
            println("Name is $name")
        }
    }

    val book = Book("Where The Wild Things Are", 9.99)
    val discountPrice = book.let {
        book.price * 0.9
    }
    println("discount price is $discountPrice")
}

// use "with" only on non-nullable receivers, and when you don't need its result
fun withFunction() {
    val book = Book("Where The Wild Things Are", 9.99)
    with(book) {
        println("book title: $title")
        println("book price: $price")
    }
}

fun alsoOperator() {
    val firstName = "Tom"
    val secondName = "Michael"
    val names = listOf(firstName, null, secondName)

    names.forEach { name ->
        name?.also {
            println("Name is $name")
        }
    }
}

fun main() {
    applyFunction()
    alsoFunction()
    letFunction()
    withFunction()
}
