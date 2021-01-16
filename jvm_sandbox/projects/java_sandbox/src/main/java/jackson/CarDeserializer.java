package jackson;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import java.io.IOException;

// This customized deserializer always increases the number of doors by 1
public class CarDeserializer extends StdDeserializer<Car> {

    protected CarDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public Car deserialize(JsonParser parser, DeserializationContext ctx) throws IOException, JsonProcessingException {
        Car car = new Car();
        while (!parser.isClosed()) {
            JsonToken jsonToken = parser.nextToken();
            if (JsonToken.FIELD_NAME.equals(jsonToken)) {
                String fieldName = parser.getCurrentName();
                System.out.println("field name: " + fieldName);

                parser.nextToken();
                if ("brand".equals(fieldName)) {
                    car.setBrand(parser.getValueAsString());
                } else if ("doors".equals(fieldName)) {
                    System.out.println("increase doors by 1");
                    car.setDoors(parser.getValueAsInt() + 1);
                }
            }
        }

        return car;
    }
}
