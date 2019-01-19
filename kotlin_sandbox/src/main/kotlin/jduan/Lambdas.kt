package jduan.lambdas

import java.io.File

data class Person(val name: String, val age: Int)

fun findOldestPerson(): Person? {
    val people = listOf(Person("Alice", 29), Person("Bob", 33))
    // return people.maxBy { it.age }
    return people.maxBy(Person::age) // member reference
}

fun lambdaExamples() {
    // bind a lambda to a variable
    val sum = {x: Int, y: Int -> x + y}
    println(sum(1, 2))

    // define a lambda and call it immediately
    // { println(42) }()

    // This is more readable!
    run { println(42) }

    val people = listOf(Person("Alice", 29), Person("Bob", 33))
    val names = people.joinToString(separator = " ", transform = {p: Person -> p.name})
    // or
    val names2 = people.joinToString(separator = " ") {p: Person -> p.name}
    // the type of 'p' can be inferred
    val names3 = people.joinToString(separator = " ") {p -> p.name}
    val names4 = people.joinToString(separator = " ") {it.name}
    println(names)
    println(names2)
    println(names3)
    println(names4)
}

// demonstrate "lexical scope"
fun printMessages(messages: Collection<String>, prefix: String) {
    messages.forEach {
        println("$prefix $it")
    }
}

// demonstrate that variables in lexical scope can be modified
fun printProblemCounts(responses: Collection<String>) {
    var clientErrors = 0
    var serverErrors = 0
    responses.forEach {
        if (it.startsWith("4")) {
            clientErrors++
        } else if (it.startsWith("5")) {
            serverErrors++
        }
    }
    println("$clientErrors client errors, $serverErrors server errors")
}

class Book(val title: String, val authors: List<String>)

fun collectionExamples() {
    // filter
    val numbers = listOf(1, 2, 3, 4)
    println(numbers.filter { it % 2 == 0 })
    val people = listOf(Person("Alice", 29), Person("Bob", 31), Person("Carol", 35))
    println(people.filter { it.age > 30 })

    // map
    println(numbers.map { it * it })
    println(people.map { it.name })
    // println(people.map(Person::name)

    // chain them
    println(people.filter { it.age > 30 }.map(Person::name))

    val table = mapOf(0 to "zero", 1 to "one")
    // return a new map with the same keys but values are transformed
    // other functions: filterKeys, filterValues, mapKeys, mapValues
    println(table.mapValues { it.value.toUpperCase() })

    val canBeInClub27 = {p: Person -> p.age <= 27 }

    // all
    println(people.all(canBeInClub27))

    // any
    println(people.any(canBeInClub27))

    // count
    println(people.count(canBeInClub27))
    // "count" is better than "filter, then size" because it doesn't create an intermediate list
    println(people.filter(canBeInClub27).size)

    // find: return the first match or null
    // a synonym is: firstOrNull
    println(people.find(canBeInClub27))

    // groupBy
    // returns: {
    //      29=[Person(name=Alice, age=29)],
    //      31=[Person(name=Bob, age=31)],
    //      35=[Person(name=Carol, age=35)]
    // }
    println(people.groupBy { it.age })
    println(listOf("a", "ab", "b").groupBy(String::first))

    val books = listOf(
            Book("Thursday Next", listOf("Jasper Fforde")),
            Book("Mort", listOf("Terry Pratchett")),
            Book("Good Omens", listOf("Terry Pratchett", "Neil Gaiman"))
    )

    // flatMap
    println(books.flatMap { it.authors }.toSet())
}

// lazy evaluation
fun sequenceExamples() {
    val people = listOf(Person("Alice", 29), Person("Bob", 31), Person("Carol", 35))

    val names = people.asSequence()
            .map(Person::name)
            .filter { it.startsWith("A") }
            .toList()
    println(names)

    val numbers = listOf(1, 2, 3, 4).asSequence()
            .map { print("map($it) "); it * it }
            .filter { print("filter($it) "); it % 2 == 0 }
            .map { print("map($it) "); it + 1 }
            .toList()
    // it prints: all the functions are applied to one element before moving on the next element!
    // map(1) filter(1)
    // map(2) filter(4) map(4)
    // map(3) filter(9)
    // map(4) filter(16) map(16)
    // [5, 17]
    println(numbers)

    // generate sequences
    val naturalNumbers = generateSequence(0) { it + 1 }
    val numbersTo100 = naturalNumbers.takeWhile { it <= 100 }
    println(numbersTo100.sum())

    fun File.isInsideHiddenDirectory() =
            kotlin.sequences.generateSequence(this) { it.parentFile }.any { it.isHidden }

    val file = File("/Users/jingjing_duan/.vim/autoload/pathogen.vim")
    println(file.isInsideHiddenDirectory())
}

fun alphabet(): String {
    val result = StringBuilder()
    for (letter in 'A'..'Z') {
        result.append(letter)
    }
    result.append("\nNow I know the alphabet!")
    return result.toString()
}

fun lambdaWithReceivers(): String {
    val result = StringBuilder()
    return with(result) {
        for (letter in 'A' .. 'Z') {
            // you can also omit "this"
            this.append(letter)
        }
        this.append("\nNow I know the alphabet!")
        this.toString()
    }
}

fun lambdaWithReceivers2() = StringBuilder().apply {
    for (letter in 'A' .. 'Z') {
        append(letter)
    }
    append("\nNow I know the alphabet!")
}.toString()

// buildString does the following:
// StringBuilder().apply(builderAction).toString()
fun alphabet2() = buildString {
    for (letter in 'A' .. 'Z') {
        append(letter)
    }
    append("\nNow I know the alphabet!")
}
