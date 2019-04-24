import org.scalatest.FunSuite
import org.junit.runner.RunWith
import org.scalatest.junit.JUnitRunner

@RunWith(classOf[JUnitRunner])
class MyScalaSpec extends FunSuite {
    // This test shows how to exclude tests with spaces in the name.
    test("Basic logic. Pretty cool") {
         assert(1 == 2, "1 and 2 are equal")
    }
}
