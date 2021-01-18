package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.ObjectReader;
import java.io.IOException;
import java.util.Iterator;
import org.junit.Test;

/**
 * Jackson has a built-in tree model which can be used to represent a JSON object. Jackson's tree
 * model is useful if you don't know how the JSON you will receive looks, or if you for some reason
 * cannot (or just don't want to) create a class to represent it. The Jackson Tree Model is also
 * useful if you need to manipulate the JSON before using or forwarding it. All of these situations
 * can easily occur in a Data Streaming scenario.
 */
public class JsonNodeTest {
    @Test
    public void testSimple() throws IOException {
        String carJson = "{ \"brand\" : \"Mercedes\", \"doors\" : 5 }";
        ObjectMapper objectMapper = new ObjectMapper();
        // You can also do this:
        // JsonNode jsonNode = objectMapper.readTree(carJson);
        JsonNode jsonNode = objectMapper.readValue(carJson, JsonNode.class);

        assertEquals("Mercedes", jsonNode.get("brand").asText());
    }

    @Test
    public void accessJsonNode() throws IOException {
        String carJson =
            "{ \"brand\" : \"Mercedes\"," +
                " \"doors\" : 5," +
                "  \"owners\" : [\"John\", \"Jack\", \"Jill\"]," +
                "  \"nestedObject\" : { \"field\" : \"value\" } }";

        ObjectMapper objectMapper = new ObjectMapper();
        JsonNode jsonNode = objectMapper.readValue(carJson, JsonNode.class);

        JsonNode brandNode = jsonNode.get("brand");
        assertEquals("Mercedes", brandNode.asText());
        assertEquals(5, jsonNode.get("doors").asInt());

        JsonNode ownersNode = jsonNode.get("owners");
        assertEquals("John", ownersNode.get(0).asText());

        JsonNode childNode = jsonNode.get("nestedObject");
        assertEquals("value", childNode.get("field").asText());
    }

    @Test
    public void javaObjectToJsonNode() {
        ObjectMapper objectMapper = new ObjectMapper();

        Car car = new Car("Cadillac", 4);
        JsonNode jsonNode = objectMapper.valueToTree(car);

        assertEquals("Cadillac", jsonNode.get("brand").asText());
    }

    @Test
    public void testJsonPath() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String jsonString = "{" +
            "\"person\": {" +
            "\"name\": \"James\"," +
            "\"ssn\": \"abc123\"" +
            "}}";

        JsonNode node = objectMapper.readTree(jsonString);
        // JsonNode.at takes a JSON "path expression", which specifies the complete path from the
        // root JsonNode and down to the field you want to access the value of.
        assertEquals("James", node.at("/person/name").asText());
        assertEquals("abc123", node.at("/person/ssn").asText());
    }

    @Test
    public void testListOfObjects() throws IOException {
        ObjectMapper objectMapper = new ObjectMapper();
        String carsString = "[" +
            "{ \"brand\" : \"Mercedes\", \"doors\" : 5 }, " +
            "{ \"brand\" : \"Honda\", \"doors\" : 4 }, " +
            "{ \"brand\" : \"Toyota\", \"doors\" : 3 }" +
            "]";
        JsonNode node = objectMapper.readTree(carsString);
        for (Iterator<JsonNode> it = node.iterator(); it.hasNext(); ) {
            JsonNode child = it.next();
            JsonNode doors = child.get("doors");
            System.out.println("doors: " + doors.asText());
        }
    }
}
