import static junit.framework.TestCase.assertEquals;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

public class MyJavaTest {
  @Before
  public void setup() {
    System.out.println("Current time: " + System.currentTimeMillis());
  }

  @After
  public void teardown() {
    // If you comment out this line, you will get a PMD violation. See [EnsureResourceCleanup]
    System.out.println("Current time: " + System.currentTimeMillis());
  }

  @Test
  public void hello1() {
    assertEquals("1 should be equal to 1", 1, 1);
  }
}
