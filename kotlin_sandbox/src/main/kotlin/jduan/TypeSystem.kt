package jduan.types

fun strLen(s: String) = s.length

fun strLenSafe(s: String?) = s?.length ?: 0

fun runTypes() {
  println(strLen("hello"))
  // the line below won't compilei
  // println(strLen(null))
  println(strLenSafe(null))
  println(strLenSafe("world"))
}

fun printAllCaps(s: String?) {
  val allCaps: String? = s?.toUpperCase()
  println(allCaps)
}

class Employee(val name: String, val manager: Employee?)

// Safe calls can be used for accessing properties as well, not just for method calls.
fun managerName(employee: Employee): String? = employee.manager?.name

fun testSafeCall() {
  printAllCaps("hello")
  printAllCaps(null)

  val ceo = Employee("Da Boss", null)
  val developer = Employee("Bob Smith", ceo)

  println(managerName(ceo))
  println(managerName(developer))
}

fun letExample() {
  fun sendEmailTo(email: String) {
    println("sending email to $email")
  }

  var email: String? = "yole@example.com"
  email?.let { sendEmailTo(it) }

  email = null
  // nothing happens if email is null
  email?.let { sendEmailTo(it) }
}

fun verifyUserInput(input: String?) {
  if (input.isNullOrBlank()) {
    println("Please fill in the required fields")
  }
}
