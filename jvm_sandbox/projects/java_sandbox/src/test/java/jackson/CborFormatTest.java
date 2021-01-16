package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.cbor.CBORFactory;
import java.io.IOException;
import org.junit.Test;

// Jackson can serialize objects to CBOR format.
public class CborFormatTest {
    @Test
    public void simple() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper(new CBORFactory());

        Car car = new Car("Mercedes", 4);
        byte[] bytes = objectMapper.writeValueAsBytes(car);
        assertEquals(24, bytes.length);

        // This shows you can read it back from the bytes
        Car car2 = objectMapper.readValue(bytes, Car.class);
        assertEquals("Mercedes", car2.getBrand());
        assertEquals(4, car2.getDoors());
    }
}
