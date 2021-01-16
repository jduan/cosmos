package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import org.junit.Test;

/**
 * Jackson has a built-in tree model which can be used to represent a JSON object.
 * Jackson's tree model is useful if you don't know how the JSON you will receive
 * looks, or if you for some reason cannot (or just don't want to) create a class
 * to represent it. The Jackson Tree Model is also useful if you need to manipulate
 * the JSON before using or forwarding it. All of these situations can easily occur
 * in a Data Streaming scenario.
 */
public class JsonNodeTest {
    @Test
    public void testSimple() throws IOException {
        String carJson = "{ \"brand\" : \"Mercedes\", \"doors\" : 5 }";
        ObjectMapper objectMapper = new ObjectMapper();
        // You can also do this:
        // JsonNode jsonNode = objectMapper.readTree(carJson);
        JsonNode jsonNode = objectMapper.readValue(carJson, JsonNode.class);
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
}
