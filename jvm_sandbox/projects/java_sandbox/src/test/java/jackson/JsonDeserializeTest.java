package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.core.Version;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.module.SimpleModule;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import org.junit.Test;

public class JsonDeserializeTest {
    @Test
    public void testDeserialization() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String carJson =
            "{ \"brand\": \"Mercedes\", \"doors\": 5 }";
        Car car = objectMapper.readValue(carJson, Car.class);
        assertEquals("Mercedes", car.getBrand());
        assertEquals(5, car.getDoors());
    }

    @Test
    // ObjectMapper.readValue supports reading from many sources:
    // * strings
    // * files
    // * URLs
    // * InputStream
    // * byte arrays
    public void testDeserializationFromFile() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        Car car = objectMapper.readValue(createTempFile().toFile(), Car.class);
        assertEquals("Mercedes", car.getBrand());
        assertEquals(4, car.getDoors());
    }

    @Test
    public void testDeserizeArrayOfObjects() throws IOException {
        String jsonArray = "[{\"brand\":\"ford\"}, {\"brand\":\"Fiat\"}]";
        ObjectMapper objectMapper = new ObjectMapper();
        Car[] cars = objectMapper.readValue(jsonArray, Car[].class);
        assertEquals(2, cars.length);
        assertEquals("ford", cars[0].getBrand());
        assertEquals(0, cars[0].getDoors());
        assertEquals("Fiat", cars[1].getBrand());
        assertEquals(0, cars[1].getDoors());
    }

    @Test
    public void testDeserizeListOfObjects() throws IOException {
        String jsonArray = "[{\"brand\":\"ford\"}, {\"brand\":\"Fiat\"}]";
        ObjectMapper objectMapper = new ObjectMapper();
        // TypeReference<List<Car>> tells jackson what classes to use
        List<Car> cars = objectMapper.readValue(jsonArray, new TypeReference<List<Car>>(){});
        assertEquals(2, cars.size());
        assertEquals("ford", cars.get(0).getBrand());
        assertEquals(0, cars.get(0).getDoors());
        assertEquals("Fiat", cars.get(1).getBrand());
        assertEquals(0, cars.get(1).getDoors());
    }

    @Test
    public void testDeserizeToMap() throws IOException {
        String jsonString = "{\"brand\":\"ford\", \"doors\":5}";
        ObjectMapper objectMapper = new ObjectMapper();
        // TypeReference<List<Car>> tells jackson what classes to use
        Map<String, Object> car = objectMapper.readValue(jsonString, new TypeReference<Map<String, Object>>(){});
        assertEquals("ford", car.get("brand"));
        assertEquals(5, car.get("doors"));
    }

    @Test
    public void ignoreUnknownFields() throws IOException {
        String jsonString = "{\"brand\":\"ford\", \"doors\":5, \"make\": \"mustang\"}";
        ObjectMapper objectMapper = new ObjectMapper();
        // Without this line, you will get an exception:
        // com.fasterxml.jackson.databind.exc.UnrecognizedPropertyException
        objectMapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        Car car = objectMapper.readValue(jsonString, Car.class);
        assertEquals("ford", car.getBrand());
        assertEquals(5, car.getDoors());
    }

    @Test
    public void testCustomDeserializer() throws IOException {
        SimpleModule module =
            new SimpleModule("CarDeserializer", new Version(3, 1, 8, null, null, null));
        module.addDeserializer(Car.class, new CarDeserializer(Car.class));

        String jsonString = "{\"brand\":\"ford\", \"doors\":5}";
        ObjectMapper objectMapper = new ObjectMapper();
        objectMapper.registerModule(module);
        Car car = objectMapper.readValue(jsonString, Car.class);
        assertEquals("ford", car.getBrand());
        assertEquals(6, car.getDoors());
    }

    private Path createTempFile() throws IOException {
        // create a temporary file
        Path tempFile = Files.createTempFile(null, null);
        // write a line
        String carJson =
            "{ \"brand\" : \"Mercedes\", \"doors\" : 4 }";
        Files.write(tempFile, carJson.getBytes(StandardCharsets.UTF_8));
        return tempFile;
    }
}
