package jackson;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.ser.std.StdSerializer;
import java.io.IOException;

// Custom serializer for Car class
public class CarSerializer extends StdSerializer<Car> {
    protected CarSerializer(Class<Car> t) {
        super(t);
    }

    @Override
    public void serialize(Car car, JsonGenerator gen, SerializerProvider provider) throws IOException {
        gen.writeStartObject();
        gen.writeStringField("producer", car.getBrand());
        gen.writeNumberField("doorCount", car.getDoors());
        gen.writeEndObject();
    }
}
