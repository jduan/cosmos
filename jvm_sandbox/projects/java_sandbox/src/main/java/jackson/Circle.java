package jackson;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.NoArgsConstructor;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@NoArgsConstructor
public class Circle implements Shape {
    @JsonProperty
    int radius;

    public Circle(int radius) {
        this.radius = radius;
    }

    public static Circle of(int r) {
        return new Circle(r);
    }

    @Override
    public String toString() {
        return "Circle{" +
            "radius=" + radius +
            '}';
    }
}
