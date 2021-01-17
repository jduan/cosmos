package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.annotation.JacksonInject;
import com.fasterxml.jackson.annotation.JsonAnySetter;
import com.fasterxml.jackson.annotation.JsonSetter;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.BeanProperty;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.InjectableValues;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.ObjectReader;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import lombok.Getter;
import lombok.Setter;
import org.junit.Test;

// Jackson contains a set of annotations that only affect how Jackson parses JSON into objects -
// meaning they affect Jackson's reading of JSON.
public class ReadAnnotationsTest {
    @Test
    public void testJsonSetter() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = "{\"id\": 1234, \"name\": \"John\"}";

        Person person = objectMapper.readValue(jsonString, Person.class);
        assertEquals(1234, person.getPersonId());
        assertEquals("John", person.getName());
    }

    @Test
    public void testJsonAnySetter() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = "{\"id\": 1234, \"name\": \"John\"}";

        Bag bag = objectMapper.readValue(jsonString, Bag.class);
        assertEquals(1234, bag.get("id"));
        assertEquals("John", bag.get("name"));
    }

    @Test
    public void testJsonCreator() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = "{\"id\": 1234, \"name\": \"John\"}";

        PersonImmutable person = objectMapper.readValue(jsonString, PersonImmutable.class);
        assertEquals(1234, person.getId());
        assertEquals("John", person.getName());
    }

    @Test
    public void testJsonInject() throws IOException {
        InjectableValues inject = new InjectableValues.Std().addValue(String.class, "google.com");
        ObjectReader objectReader = new ObjectMapper().reader(inject).forType(PersonInject.class);
        String jsonString = "{\"id\": 1234, \"name\": \"John\"}";

        PersonInject person = objectReader.readValue(jsonString);
        assertEquals(1234, person.getId());
        assertEquals("John", person.getName());
        assertEquals("google.com", person.getSource());
    }

    @Test
    public void testJsonDeserialize() throws IOException {
        String jsonString = "{\"id\": 1234, \"name\": \"John\", \"enabled\": \"1\"}";
        ObjectReader objectReader = new ObjectMapper().readerFor(PersonDeserialize.class);

        PersonDeserialize person = objectReader.readValue(jsonString);
        assertEquals(1234, person.getId());
        assertEquals("John", person.getName());
        assertEquals(true, person.isEnabled());
    }

    @Getter
    private static class Person {
        private long   personId = 0;
        @Setter
        private String name     = null;

        // @JsonSetter is used to tell jackson what should match the name of this property
        // in the JSON data.
        @JsonSetter("id")
        public void setPersonId(long personId) { this.personId = personId; }
    }

    @Getter
    @Setter
    private static class PersonInject {
        private long   id = 0;
        private String name     = null;

        @JacksonInject
        private String source = null;
    }

    @Getter
    @Setter
    private static class PersonDeserialize {
        private long   id = 0;
        private String name     = null;

        // specify a custom deserializer for a field
        @JsonDeserialize(using = OptimizedBooleanDeserializer.class)
        private boolean enabled = false;
    }

    private static class OptimizedBooleanDeserializer extends JsonDeserializer<Boolean> {

        @Override
        public Boolean deserialize(JsonParser parser, DeserializationContext ctxt) throws IOException, JsonProcessingException {
            String text = parser.getText();
            if ("0".equals(text)) {
                return false;
            } else {
                return true;
            }
        }
    }

    private static class Bag {
        private Map<String, Object> properties = new HashMap<>();

        // @JsonAnySetter tells jackson to call this method for "all unrecognized" fields in
        // the JSON string.
        @JsonAnySetter
        public void set(String name, Object value) {
            this.properties.put(name, value);
        }

        public Object get(String name) {
            return this.properties.get(name);
        }
    }
}
