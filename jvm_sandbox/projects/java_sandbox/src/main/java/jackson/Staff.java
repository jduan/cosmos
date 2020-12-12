package jackson;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import java.math.BigDecimal;
import java.util.List;
import java.util.Map;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
@JsonPropertyOrder({"name", "age", "position", "skills", "salary"})
// @JsonInclude can also be used for individual properties or globally (by changing the ObjectMapper)
@JsonInclude(JsonInclude.Include.NON_NULL)
public class Staff {
    @JsonProperty("custom_name")
    private String name;
    private int age;
    private String[] position;
    private List<String> skills;
    private Map<String, BigDecimal> salary;

    @JsonIgnore
    private String secret;
}
