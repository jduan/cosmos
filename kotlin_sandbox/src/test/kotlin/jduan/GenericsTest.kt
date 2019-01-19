package jduan.generics

import kotlin.test.assertEquals
import org.junit.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class GenericsTest {
  @Test fun test0() {
    runGenerics()
  }

  @Test fun test1() {
    val letters = ('a'..'z').toList()
    assertEquals(listOf('a', 'b', 'c'), letters.slice<Char>(0..2))
  }

  @Test fun test2() {
    val authors = listOf("Dmitry", "Svetlana")
    val readers = mutableListOf<String>("Jingjing", "John")
    assertEquals(listOf("Jingjing", "John"), readers.filter { it !in authors })
  }

  @Test fun test3() {
    assertEquals(3, listOf(1, 2, 3, 4).penultimate)
    assertEquals(1.5, oneHalf(3.0))
    assertEquals("kotlin", max("kotlin", "java"))
    // The line below won't compile!
    // assertEquals("kotlin", max("kotlin", 42))
  }

  @Test fun test4() {
    val helloWorld = StringBuilder("Hello World")
    ensureTrailPeriod(helloWorld)
    assertEquals("Hello World.", helloWorld.toString())
  }

  // reified type parameters@
  // In this case, the type argument is known at runtime, and filterIsInstance
  // uses it to check which values in the list are instances of the class
  // specified as the type argument.
  @Test fun test5() {
    assertTrue(isA<String>("abc"))
    assertFalse(isA<String>(123))
    val items = listOf("one", 2, "three")
    assertEquals(listOf("one", "three"), items.filterIsInstance<String>())
  }

  @Test fun test6() {
    printContents(listOf("abc", "bac"))
    val strings = mutableListOf("abc", "bac")
    // the line below won't compile!
    // addAnswer(strings)
  }

  @Test fun test7() {
    runValidators()
    runValidators2()
  }
}
