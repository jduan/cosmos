package jackson;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.NoArgsConstructor;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@NoArgsConstructor
public class Rectangle implements Shape {
    @JsonProperty
    private int w;
    @JsonProperty
    private int h;

    public Rectangle(int w, int h) {
        this.w = w;
        this.h = h;
    }

    public static Rectangle of(int w, int h) {
        return new Rectangle(w, h);
    }

    @Override
    public String toString() {
        return "Rectangle{" +
            "w=" + w +
            ", h=" + h +
            '}';
    }
}
