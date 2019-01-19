import org.junit.Test;
import static org.junit.Assert.*;

public class HelloTest {
    @Test public void testHello() {
        assertEquals("world", new Hello().hello());
    }
}
