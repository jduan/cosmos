package jduan.main

import jduan.geometry.shapes.createRandomRectangle
import jduan.geometry.colors.*
import jduan.conventions.runConventions
import jduan.generics.runGenerics
import jduan.hof.runHOF
import jduan.lambdas.*
import jduan.types.*
import jduan.exceptions.readNumber
import jduan.expr.*;
import jduan.loop.*;
import jduan.collections.*;
import jduan.geometry.shapes.Rectangle
import jduan.interfaces.*;
import jduan.objects.Payroll
import org.ocpsoft.prettytime.PrettyTime
import jduan.extensions.lastChar as last;
import java.io.BufferedReader
import java.io.StringReader
import java.util.*

fun main(args: Array<String>) {
  val persons = listOf(Person("Alice", 30),
      Person(name = "John", age = 99),
      Person("Bob", age = 29))
  // "it" is the default name of lambda parameter
  // "?:" is called the Elvis operator
  val oldest = persons.maxBy { it.age ?: 0 }
  println("The oldest is: $oldest")

  val rect = Rectangle(3, 4)
  println(rect.isSquare)
  println(createRandomRectangle().isSquare)
  println(getMnemonic(Color.BLUE))
  println(getMnemonic(Color.BLUE))
  println(mix(Color.BLUE, Color.YELLOW))
  println(eval(Sum(Num(1), Num(2))))
  println(eval2(Sum(Num(1), Num(2))))
  testLoops()
  iterateMap()
  iterateList()
  println(recognize('$'))
  readNumber(BufferedReader(StringReader("not a number")))

  // collections
  val set = hashSetOf(1, 7, 53)
  val list = arrayListOf(1, 7, 53)
//    val map = hashMapOf(1 to "one", 7 to "seven", 53 to "fifty-three")
  println("is 1 in the set? ${1 in set}")
  println("last element of list: ${list.last()}")
  println("max element of set: ${set.max()}")

  println(list.joinToString())

  println("Kotlin".last())

  // extension properties
  val sb = StringBuilder("Kotlin?")
  sb.lastChar = '!'
  println(sb)

  Button().showOff()

  println(TwitterUser("Jingjing", "Duan").nickname)

  Payroll.addEmployee(jduan.objects.Person("John", 30.0))
  Payroll.addEmployee(jduan.objects.Person("Mike", 40.0))
  println(Payroll.calculateSalary())

  println(findOldestPerson())
  println(lambdaExamples())
  val errors = listOf("403 Forbidden", "404 Not Found")
  printMessages(errors, "Error:")
  val responses = listOf("200 OK", "418 I'm a teapot", "500 Internal Server Error")
  printProblemCounts(responses)
  collectionExamples()
  sequenceExamples()
  println(lambdaWithReceivers())
  println(lambdaWithReceivers2())

  runTypes()
  printAllCaps("abc")
  printAllCaps(null)
  testSafeCall()
  letExample()
  verifyUserInput(null)
  verifyUserInput("  ")

  runConventions()

  runHOF()

  runGenerics()

  println(PrettyTime(Locale("en/US")).format(Date(-1)))
}

