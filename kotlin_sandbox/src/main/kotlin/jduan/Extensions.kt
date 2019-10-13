package jduan

fun String.lastChar(): Char = this[(this.length - 1)]

fun main() {
    val name = "hello"
    println("last char of string $name is ${name.lastChar()}")
}
