package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonIgnoreType;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import org.junit.Test;

/**
 * Jackson has a list of annotations that influence both reading of Java objects from JSON, as well
 * as the writing of Java objects into JSON.
 */
public class ReadWriteAnnotationsTest {
    @Test
    public void testJsonIgnore() throws IOException {
        Person person = new Person(1, "John Wick");
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = objectMapper.writeValueAsString(person);
        String expectedString = "{\"name\":\"John Wick\"}";
        assertEquals(expectedString, jsonString);

        Person person2 = objectMapper.readValue(jsonString, Person.class);
        assertEquals("John Wick", person2.getName());
        // Because the serialized string doesn't contain the id, we can't read it back!
        assertEquals(0, person2.getPersonId());
    }

    @Test
    public void testJsonIgnoreProperties() throws IOException {
        Car car = new Car("Mercedes", "GL450", 2020, 10000);
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = objectMapper.writeValueAsString(car);
        String expectedString = "{\"year\":2020,\"mileage\":10000}";
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonIgnoreType() throws IOException {
        PersonIgnoreType person = new PersonIgnoreType(1, "John Wick",
            new Address("Main st", "Boston"));
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = objectMapper.writeValueAsString(person);
        String expectedString = "{\"name\":\"John Wick\"}";
        assertEquals(expectedString, jsonString);
    }

    @NoArgsConstructor
    @AllArgsConstructor
    @Getter
    @Setter
    private static class Person {
        @JsonIgnore
        public long personId = 0;
        public String name = null;
    }

    @NoArgsConstructor
    @AllArgsConstructor
    @Getter
    @Setter
    // You can ignore multiple properties
    @JsonIgnoreProperties({"brand", "make"})
    private static class Car {
        public String brand = null;
        public String make = null;
        public long year;
        public long mileage;
    }

    @NoArgsConstructor
    @AllArgsConstructor
    @Getter
    @Setter
    private static class PersonIgnoreType {
        @JsonIgnore
        public long personId = 0;
        public String name = null;
        private Address address;
    }

    @AllArgsConstructor
    // This marks this class to be ignored everywhere it's used.
    @JsonIgnoreType
    private static class Address {
        private String name;
        private String city;
    }
}
