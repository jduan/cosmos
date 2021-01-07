package jackson;

import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.jsontype.NamedType;
import com.sun.tools.javac.util.List;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Set;
import java.util.stream.Collectors;
import org.reflections.Reflections;

public class JacksonExample2 {
    public static void main(String[] args) throws IOException {
        View v = new View();
        v.setShapes(new ArrayList<>(List.of(Rectangle.of(3, 6), Circle.of(5))));

        System.out.println("-- serializing --");
        ObjectMapper om = new ObjectMapper();
        registerSubTypes(om);
        String s = om.writeValueAsString(v);
        System.out.println(s);

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
