package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.core.Version;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.module.SimpleModule;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;
import org.junit.Test;

public class JsonSerializeTest {
    @Test
    public void testWriteValue() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        Car car = new Car();
        car.setBrand("BMW");
        car.setDoors(4);
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        objectMapper.writeValue(out, car);

        String jsonString = "{\"brand\":\"BMW\",\"doors\":4}";
        assertEquals(jsonString, out.toString());
    }

    @Test
    public void testWriteValueAsString() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        Car car = new Car();
        car.setBrand("BMW");
        car.setDoors(4);
        String jsonString = objectMapper.writeValueAsString(car);

        String expectedJsonString = "{\"brand\":\"BMW\",\"doors\":4}";
        assertEquals(expectedJsonString, jsonString);
    }

    @Test
    public void testWriteValueAsBytes() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        Car car = new Car();
        car.setBrand("BMW");
        car.setDoors(4);
        byte[] bytes = objectMapper.writeValueAsBytes(car);

        String expectedJsonString = "{\"brand\":\"BMW\",\"doors\":4}";
        assertEquals(expectedJsonString, new String(bytes));
    }

    @Test
    public void testCustomSerializer() throws IOException {
        CarSerializer carSerializer = new CarSerializer(Car.class);
        ObjectMapper objectMapper = new ObjectMapper();
        SimpleModule module =
            new SimpleModule("CarSerializer", new Version(2, 1, 3, null, null, null));
        module.addSerializer(Car.class, carSerializer);
        objectMapper.registerModule(module);

        Car car = new Car();
        car.setBrand("BMW");
        car.setDoors(4);
        String jsonString = objectMapper.writeValueAsString(car);

        String expectedJsonString = "{\"producer\":\"BMW\",\"doorCount\":4}";
        assertEquals(expectedJsonString, jsonString);
    }

    // By default Jackson will serialize a java.util.Date object to its long value, which
    // is the number of milliseconds since January 1st 1970.
    @Test
    public void testSerializeDate() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        Date date = new Date();
        Transaction transaction = new Transaction("transfer", date);

        String jsonString = objectMapper.writeValueAsString(transaction);
        String expectedJsonString = String.format("{\"type\":\"transfer\",\"date\":%s}", date.getTime());
        assertEquals(expectedJsonString, jsonString);
    }

    // Tell jackson how to format a Date object
    @Test
    public void testSerializeDateText() throws IOException {
        SimpleDateFormat dateFormat = new SimpleDateFormat("yyyy-MM-dd");
        ObjectMapper objectMapper = new ObjectMapper();
        objectMapper.setDateFormat(dateFormat);

        Date date = new Date();
        Transaction transaction = new Transaction("transfer", date);

        String jsonString = objectMapper.writeValueAsString(transaction);
        String expectedJsonString = String.format("{\"type\":\"transfer\",\"date\":\"%s\"}",
            dateFormat.format(date));
        assertEquals(expectedJsonString, jsonString);
    }
}
