package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.annotation.JsonAnyGetter;
import com.fasterxml.jackson.annotation.JsonGetter;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import com.fasterxml.jackson.annotation.JsonRawValue;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import lombok.AllArgsConstructor;
import org.junit.Test;

// Jackson also contains a set of annotations that can influence how Jackson serializes (writes)
// Java objects to JSON.
public class WriteAnnotationsTest {
    @Test
    public void testJsonInclude() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonInclude person = new PersonInclude(1234, "John");
        String expectedString = "{\"id\":1234,\"name\":\"John\"}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);

        PersonInclude person2 = new PersonInclude(1234, null);
        String expectedString2 = "{\"id\":1234}";
        String jsonString2 = objectMapper.writeValueAsString(person2);
        assertEquals(expectedString2, jsonString2);
    }

    @Test
    public void testJsonGetter() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonGetter person = new PersonGetter();
        person.personId = 1234;
        String expectedString = "{\"id\":1234}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonAnyGetter() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonAnyGetter person = new PersonAnyGetter();
        Map<String, Object> map = new HashMap<>();
        map.put("name", "John");
        map.put("age", 99);
        person.properties = map;
        String expectedString = "{\"name\":\"John\",\"age\":99}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonPropertyOrder() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonPropertyOrder person = new PersonPropertyOrder();
        person.id = 1234;
        person.name = "John";
        String expectedString = "{\"name\":\"John\",\"id\":1234}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonRawValue() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonRawValue person = new PersonRawValue(0, "{\"street\":\"Wall Street\",\"no\":1}");

        String expectedString = "{\"id\":0,\"address\":{\"street\":\"Wall Street\",\"no\":1}}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonValue() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonValue person = new PersonValue(0, "John");

        String expectedString = "\"0,John\"";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testJsonSerializer() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        PersonSerializer person = new PersonSerializer(0, true, "John");

        String expectedString = "{\"id\":0,\"female\":1,\"name\":\"John\"}";
        String jsonString = objectMapper.writeValueAsString(person);
        assertEquals(expectedString, jsonString);
    }

    @JsonInclude(JsonInclude.Include.NON_EMPTY)
    @AllArgsConstructor
    private static class PersonInclude {
        public long id = 0;
        public String name = null;
    }

    private static class PersonGetter {
        public long personId = 0;

        // Used to provide a different field name when serializing data
        @JsonGetter("id")
        public long personId() {
            return this.personId;
        }
    }

    private static class PersonAnyGetter {
        private Map<String, Object> properties;

        // Tell jackson to serialize a map of properties
        @JsonAnyGetter
        public Map<String, Object> getProperties() {
            return this.properties;
        }
    }

    // Normally Jackson would have serialized the properties in PersonPropertyOrder in the sequence
    // they are found in the class.
    @JsonPropertyOrder({"name", "id"})
    private static class PersonPropertyOrder {
        public long id = 0;
        public String name = null;
    }

    @AllArgsConstructor
    private static class PersonRawValue {
        public long id = 0;

        // If the value is already in desired json format, you don't want to add extra quotes.
        @JsonRawValue
        public String address = null;
    }

    @AllArgsConstructor
    private static class PersonValue {
        public long id = 0;
        public String name = null;

        // Tell jackson that the return value of this method should be used as the JSON string.
        @JsonValue
        public String toJson() {
            return this.id + "," + this.name;
        }
    }

    @AllArgsConstructor
    private static class PersonSerializer {
        public long id = 0;
        @JsonSerialize(using = OptimizedBooleanSerializer.class)
        public boolean female = false;
        public String name = null;
    }

    private static class OptimizedBooleanSerializer extends JsonSerializer<Boolean> {

        @Override
        public void serialize(Boolean value, JsonGenerator gen, SerializerProvider serializers) throws IOException, JsonProcessingException {
            if (value) {
                gen.writeNumber(1);
            } else {
                gen.writeNumber(0);
            }
        }
    }
}
