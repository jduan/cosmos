package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import java.io.IOException;
import org.junit.Test;

// Jackson can serialize objects to CBOR format.
public class YamlFormatTest {
    @Test
    public void simple() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper(new YAMLFactory());

        Car car = new Car("Mercedes", 4);
        String yamlString = objectMapper.writeValueAsString(car);
        String expectedString = "---\n" +
            "brand: \"Mercedes\"\n" +
            "doors: 4\n";
        assertEquals(expectedString, yamlString);

        // This shows you can read it back
        Car car2 = objectMapper.readValue(yamlString, Car.class);
        assertEquals("Mercedes", car2.getBrand());
        assertEquals(4, car2.getDoors());
    }
}

