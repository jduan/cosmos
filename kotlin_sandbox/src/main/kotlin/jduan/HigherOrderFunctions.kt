package jduan.hof

fun runHOF() {
  twoAndThree {a, b -> a + b}
  twoAndThree {a, b -> a * b}
  println("abc".filter { it in 'a'..'z' })

  val letters = listOf("Alpha", "Beta")
  println(letters.joinToString())
  println(letters.joinToString { it.toLowerCase() })
  println(letters.joinToString(separator = "! ") { it.toLowerCase() })

  val calculator = getShippingCostCalculator(Delivery.EXPEDITED)
  println("Shipping costs ${calculator(Order(3))}")
  val calculator2 = getShippingCostCalculator(Delivery.STANDARD)
  println("Shipping costs ${calculator2(Order(3))}")

  val contacts = listOf(
      Person("Dmitry", "Jemerov", "123-4567"),
      Person("Svetlana", "Isakova", null))

  val contactListFilters = ContactListFilters()
  with (contactListFilters) {
    prefix = "Dm"
    onlyWithPhoneNumber = true
  }
  println(contacts.filter(contactListFilters.getPredicate()))

  println(log.averageDurationFor { it.os in setOf(OS.ANDROID, OS.IOS) })
  println(log.averageDurationFor { it.os == OS.IOS && it.path == "/signup"} )
}

val sum: (Int, Int) -> Int = { x: Int, y: Int -> x + y }

val action: () -> Unit = { println(42) }

fun performRequest(
    url: String,
    callback: (code: Int, content: String) -> Unit
) {
  // TODO
}

// You can call it like this:
// val url = "http://kotl.in"
// performRequest(url) { code, content -> ... }

fun twoAndThree(operation: (Int, Int) -> Int) {
  val result = operation(2, 3)
  println("The result is $result")
}

fun String.filter(predicate: (Char) -> Boolean): String {
  val sb = StringBuffer()
  for (index in 0 until length) {
    val element = get(index)
    if (predicate(element)) sb.append(element)
  }

  return sb.toString()
}

fun <T> Collection<T>.joinToString(
    separator: String = ", ",
    prefix: String = "(",
    suffix: String = ")",
    transform: (T) -> String = { it.toString() }
): String {
  val result = StringBuilder(prefix)
  for ((index, element) in this.withIndex()) {
    if (index > 0) result.append(separator)
    result.append(transform(element))
  }

  result.append(suffix)
  return result.toString()
}

enum class Delivery {
  STANDARD,
  EXPEDITED
}

class Order(val itemCount: Int)

// This function takes a "delivery" and returns another function, which takes an Order
// and returns a Double (shipping cost).
fun getShippingCostCalculator(delivery: Delivery): (Order) -> Double {
  if (delivery == Delivery.EXPEDITED) {
    return { order -> 6 + 2.1 * order.itemCount }
  }

  return { order -> 1.2 * order.itemCount }
}

data class Person(
    val firstName: String,
    val lastName: String,
    val phoneNumber: String?
)

class ContactListFilters {
  var prefix = ""
  var onlyWithPhoneNumber = false

  fun getPredicate(): (Person) -> Boolean {
    val startsWithPrefix = { p: Person ->
      p.firstName.startsWith(prefix) || p.lastName.startsWith(prefix)
    }

    if (!onlyWithPhoneNumber) {
      // return a variable of a function type
      return startsWithPrefix
    }

    // return a lambda from this function
    return { startsWithPrefix(it) && it.phoneNumber != null }
  }
}

data class SiteVisit(
    val path: String,
    val duration: Double,
    val os: OS
)

enum class OS { WINDOWS, LINUX, MAC, IOS, ANDROID }

val log = listOf(
    SiteVisit("/", 34.0, OS.WINDOWS),
    SiteVisit("/", 22.0, OS.MAC),
    SiteVisit("/login", 12.0, OS.WINDOWS),
    SiteVisit("/signup", 8.0, OS.IOS),
    SiteVisit("/", 16.3, OS.ANDROID)
)

val averageWindowsDuration = log
    .filter { it.os == OS.WINDOWS }
    .map(SiteVisit::duration)
    .average()

fun List<SiteVisit>.averageDurationFor(predicate: (SiteVisit) -> Boolean) =
    filter(predicate).map(SiteVisit::duration).average()
