package jackson;

import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.jsontype.NamedType;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import com.sun.tools.javac.util.List;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;
import org.reflections.Reflections;

public class JacksonExample2 {
    public static void main(String[] args) throws IOException {
        View v = new View();
        v.setShapes(new ArrayList<>(List.of(Rectangle.of(3, 6), Circle.of(5))));

        System.out.println("-- serializing --");
        ObjectMapper om = new ObjectMapper();
        // Jdk8Module handles JDK datatypes like Optional.
        om.registerModule(new Jdk8Module());
        registerSubTypes(om);
        String s = om.writeValueAsString(v);
        System.out.println(s);

        Optional<Rectangle> op = Optional.of(Rectangle.of(3, 4));
        System.out.println(om.writeValueAsString(op));
        System.out.println(om.writeValueAsString(op.get()));

        System.out.println("-- deserializing --");
        View view = om.readValue(s, View.class);
        System.out.println(view);
    }

    private static void registerSubTypes(ObjectMapper mapper) {
        Set<Class> subtypes = new Reflections("jackson")
            .getTypesAnnotatedWith(JsonTypeInfo.class)
            .stream()
            .filter(type -> !type.isInterface())
            .collect(Collectors.toSet());

        subtypes.forEach( type -> {
                NamedType t = new NamedType(type, type.getSimpleName());
                mapper.registerSubtypes(t);
            }
        );
    }
}
