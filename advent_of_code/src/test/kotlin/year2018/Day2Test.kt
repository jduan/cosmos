package year2018.day2

import kotlin.test.assertEquals
import org.junit.Test

class Day2Test {
  @Test fun test1() {
    assertEquals("eeeefggkkorss", sortString("geeksforgeeks"))
    assertEquals("abccdefghhiklmnoppqrtuvxyz", sortString("hrzhyfdplumeqvaptbiocngkxc"))
  }

  @Test fun test2() {
    assertEquals(Pair(1, 1), countID("geeksforgeeks"))
    assertEquals(Pair(1, 0), countID("hello"))
    assertEquals(Pair(0, 0), countID("world"))
    assertEquals(Pair(0, 1), countID("geekse"))
    assertEquals(Pair(1, 0), countID("+hrzhyfdplumeqvaptbiocngkxc"))
    assertEquals(Pair(1, 1), countID("cdefghijkllmnopqrstuuuvwxy"))
    assertEquals(Pair(1, 1), countID("abcdfghjklmnoopqrtuvwxyzzz"))

  }

  @Test fun test3() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day2Input.txt"
    assertEquals(4940, checksum(inputFile))
  }

  @Test fun test4() {
    assertEquals(4, distance("hello", "world"))
    assertEquals(2, distance("abcde", "axcye"))
    assertEquals(1, distance("fghij", "fguij"))
  }

  @Test fun test5() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day2Input.txt"
    assertEquals("wrziyfdmlumeqvaatbiosngkc", findCommonLetters(inputFile))
  }
}
