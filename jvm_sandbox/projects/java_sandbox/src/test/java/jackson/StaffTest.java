package jackson;

import static org.junit.Assert.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.math.BigDecimal;
import java.net.URISyntaxException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import org.junit.Test;

public class StaffTest {
    private ObjectMapper mapper = new ObjectMapper();

    @Test
    public void testSerialization() throws IOException, URISyntaxException {
        Staff staff = createStaff();
        String jsonString = mapper.writeValueAsString(staff);
        String expectedString = "{\"custom_name\":\"Jingjing\",\"age\":99,\"position\":[\"Founder\",\"CTO\",\"Writer\"],\"skills\":[\"java\",\"python\",\"node\",\"kotlin\"],\"salary\":{\"2018\":14000,\"2012\":12000,\"2010\":10000}}";
        assertEquals(expectedString, jsonString);

        // pretty format
        String jsonString2 = mapper.writerWithDefaultPrettyPrinter().writeValueAsString(staff);
        java.net.URL url = this.getClass().getClassLoader().getResource("jackson/PrettyStaff.txt");
        java.nio.file.Path resPath = java.nio.file.Paths.get(url.toURI());
        String expectedString2 = new String(java.nio.file.Files.readAllBytes(resPath), "UTF8");
        assertEquals(expectedString2, jsonString2);
    }

    @Test
    public void testJsonIncludeNonNull() throws IOException {
        Staff staff = createStaff();
        staff.setSalary(null);
        String jsonString = mapper.writeValueAsString(staff);
        String expectedString = "{\"custom_name\":\"Jingjing\",\"age\":99,\"position\":[\"Founder\",\"CTO\",\"Writer\"],\"skills\":[\"java\",\"python\",\"node\",\"kotlin\"]}";
        assertEquals(expectedString, jsonString);
    }

    @Test
    public void testDeserialization() throws IOException {
        String jsonString = "{\"custom_name\":\"Jingjing\",\"age\":99,\"position\":[\"Founder\",\"CTO\",\"Writer\"],\"skills\":[\"java\",\"python\",\"node\",\"kotlin\"],\"salary\":{\"2018\":14000,\"2012\":12000,\"2010\":10000}}";
        Staff staff = mapper.readValue(jsonString, Staff.class);
        assertEquals("Jingjing", staff.getName());
        assertEquals(99, staff.getAge());
        assertEquals(null, staff.getSecret());
    }

    private Staff createStaff() {
        Staff staff = new Staff();
        staff.setName("Jingjing");
        staff.setAge(99);
        staff.setPosition(new String[]{"Founder", "CTO", "Writer"});
        Map<String, BigDecimal> salary = new HashMap() {{
            put("2010", new BigDecimal(10000));
            put("2012", new BigDecimal(12000));
            put("2018", new BigDecimal(14000));
        }};
        staff.setSalary(salary);
        staff.setSkills(Arrays.asList("java", "python", "node", "kotlin"));

        // This property will not be serialized
        staff.setSecret("I'm an alien!");

        return staff;
    }
}
