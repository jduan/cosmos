package jackson;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY)
// We don't need JsonSubTypes here because they are dynamically added by reflections in JacksonExample2.java
//@JsonSubTypes({
//    @JsonSubTypes.Type(value = Rectangle.class, name = "Rectangle"),
//    @JsonSubTypes.Type(value = Circle.class, name = "Circle"),
//})
public interface Shape {
}
