package jackson;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

public class PersonImmutable {
    private long   id = 0;
    private String name     = null;

    /**
     * The @JsonCreator annotation is useful in situations where the @JsonSetter annotation
     * cannot be used. For instance, immutable objects do not have any setter methods,
     * so they need their initial values injected into the constructor.
     */
    @JsonCreator
    public PersonImmutable(
        @JsonProperty("id") long id,
        @JsonProperty("name") String name
    ) {
        this.id = id;
        this.name = name;
    }

    public long getId() {
        return id;
    }

    public String getName() {
        return name;
    }
}

