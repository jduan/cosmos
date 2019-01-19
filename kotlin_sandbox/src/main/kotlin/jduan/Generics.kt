package jduan.generics

import java.lang.IllegalArgumentException
import kotlin.reflect.KClass

fun runGenerics() {
  val letters = ('a' .. 'z').toList()
  println(letters.slice<Char>(0..2))
  println(letters.slice(10..13))

  val authors = listOf("Dmitry", "Svetlana")
  val readers = mutableListOf<String>("Jingjing", "John")

  println(readers.filter { it !in authors })

  println(listOf(1, 2, 3).sum())

  println("oneHalf: ${oneHalf(4)}")

  println("max of Kotlin and Java is ${max("Kotlin", "Java")}")

  val helloWorld = StringBuilder("Hello World")
  ensureTrailPeriod(helloWorld)
  println(helloWorld)

  printSum(listOf(1, 2, 3))
  // The line below would throw an IllegalArgumentException
  // printSum(setOf(1, 2, 3))

  // The line below would throw a ClassCastException
  // You don't get an IllegalArgumentException because you can't check whether
  // the argument is a List<Int> or not at runtime. All it knows is that
  // the argument is a List (type erasure). Therefore the cast succeeds, and
  // the function "sum" is called on such a list anyway. During its execution,
  // an exception is thrown.
  // printSum(listOf("a", "b", "c"))

  printSum2(listOf(1, 2, 3))

  println(isA<String>("abc"))
  println(isA<String>(123))

  val items = listOf("one", 2, "three")
  val itemsString: List<String> = items.filterIsInstance<String>()
  println(itemsString)
}

// generic extension properties
val <T> List<T>.penultimate: T
  get() = this[size - 2]

// Number is the upper bound. In other words, T has to be a subtype of Number.
fun <T: Number> oneHalf(value: T): Double {
  return value.toDouble() / 2.0
}

// The upper bound for T is a generic type Comparable<T>.
fun <T: Comparable<T>> max(first: T, second: T): T =
    if (first > second) first else second

// Specify multiple constraints for a type parameter.
fun <T> ensureTrailPeriod(seq: T) where T : CharSequence, T : Appendable {
  if (!seq.endsWith('.'))
    seq.append('.')
}

fun printSum(c: Collection<*>) {
  // You will get warning for the line below:
  // Warning: Kotlin: Unchecked cast: Collection<*> to List<Int>
  val intList = c as? List<Int> ?: throw IllegalArgumentException("List is expected")
  println(intList.sum())
}

fun printSum2(c: Collection<Int>) {
  if (c is List<Int>) {
    println(c.sum())
  }
}

inline fun <reified T> isA(value: Any) = value is T

fun printContents(list: List<Any>) {
  println(list.joinToString())
}

fun addAnswer(list: MutableList<Any>) {
  list.add(42)
}

// This shows how "star projection" works.
interface FieldValidator<in T> {
  fun validate(input: T): Boolean
}

object DefaultStringValidator : FieldValidator<String> {
  override fun validate(input: String) = input.isNotEmpty()
}

object DefaultIntValidator : FieldValidator<Int> {
  override fun validate(input: Int) = input >= 0
}

fun runValidators() {
  val validators = mutableMapOf<KClass<*>, FieldValidator<*>>()
  validators[String::class] = DefaultStringValidator
  validators[Int::class] = DefaultIntValidator

  // The line below doesn't compile because what you get back
  // from the map is a generic FieldValidator which you can't use
  // against a string.
  // validators[String::class].validate("")

  // You get a compiler warning for the cast!
  val stringValidator = validators[String::class] as FieldValidator<String>
  println(stringValidator.validate(""))

  // You would get a java.lang.ClassCastException because of Int vs String
//  val stringValidator2 = validators[Int::class] as FieldValidator<String>
//  println(stringValidator2.validate(""))
}

// The solution below uses the same validators map but encapsulates all the access to it into two generic
// methods responsible for having only correct validators registered and returned. This code also emits a warning about
// the unchecked cast (the same one), but here the object Validators controls all access to the map, which guarantees
// that no one will change the map incorrectly.
// Now you have a type-safe API. All the unsafe logic is hidden in the body of the class; and by localizing it,
// you guarantee that it can’t be used incorrectly.
object Validators {
  private val validators = mutableMapOf<KClass<*>, FieldValidator<*>>()

  fun <T: Any> registerValidator(kClass: KClass<T>, fieldValidator: FieldValidator<T>) {
    validators[kClass] = fieldValidator
  }

  @Suppress("UNCHECKED_CAST")
  operator fun <T: Any> get(kClass: KClass<T>): FieldValidator<T> =
      validators[kClass] as? FieldValidator<T>
          ?: throw IllegalArgumentException("No validator for ${kClass.simpleName}")
}

fun runValidators2() {
  Validators.registerValidator(String::class, DefaultStringValidator)
  Validators.registerValidator(Int::class, DefaultIntValidator)
  println(Validators[String::class].validate("Kotlin"))
  println(Validators[Int::class].validate(42))
}
