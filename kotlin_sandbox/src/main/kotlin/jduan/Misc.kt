package jduan.misc

data class Person(val name: String,
    // null by default
                  val age: Int? = null)

data class Person2 (var name: String,
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
