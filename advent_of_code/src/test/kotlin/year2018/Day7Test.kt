package year2018.day7

import kotlin.test.assertEquals
import org.junit.Test

class Day7Test {
  @Test
  fun test1() {
    assertEquals(Pair("A", "L"),
            parseLine("Step A must be finished before step L can begin."))
  }

  @Test
  fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day7TestInput.txt"
    val nodes = parseFile(inputFile)
    assertEquals(1, nodes.size)
    assertEquals("C", nodes.first().name)
  }

  @Test
  fun test3() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day7TestInput.txt"
    assertEquals("CABDFE", topsort(inputFile))
  }

  // part 1
  @Test
  fun test4() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day7Input.txt"
    assertEquals("ABLCFNSXZPRHVEGUYKDIMQTWJO", topsort(inputFile))
  }

  @Test
  fun test5() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day7TestInput.txt"
    assertEquals(15, Solution2(inputFile, 2).topsort())
  }

  @Test
  fun test6() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day7Input.txt"
    assertEquals(1157, Solution2(inputFile, 5, 60).topsort())
  }
}
