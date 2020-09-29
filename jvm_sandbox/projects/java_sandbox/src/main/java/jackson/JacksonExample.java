package jackson;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;

public class JacksonExample {
    public static void main(String[] args) throws IOException {
        String rawJson =
            "{\n" +
                "\"regions\": "
            + "[\n"
                + "  {\n"
                + "    \"name\": \"R1\",\n"
                + "    \"countryStateCityList\": [\n"
                + "      \"AG\",\n"
                + "      \"MX.Quintana Roo\",\n"
                + "      \"MX.Solidaridad, Quintana Roo\"\n"
                + "    ]\n"
                + "  },\n"
                + "  {\n"
                + "    \"name\": \"R1G1\",\n"
                + "    \"parent\": \"R1\",\n"
                + "    \"countryStateCityList\": [\n"
                + "      \"AG\"\n"
                + "    ]\n"
                + "  },\n"
                + "  {\n"
                + "    \"name\": \"R2\",\n"
                + "    \"countryStateCityList\": [\n"
                + "      \"MX\",\n"
                + "      \"US.Hawaii\"\n"
                + "    ]\n"
                + "  },\n"
                + "  {\n"
                + "    \"name\": \"R4\",\n"
                + "    \"countryStateCityList\": [\n"
                + "      \"CA\",\n"
                + "      \"US\"\n"
                + "    ]\n"
                + "  },\n"
                + "  {\n"
                + "    \"name\": \"R4G1\",\n"
                + "    \"parent\": \"R4\",\n"
                + "    \"countryStateCityList\": [\n"
                + "      \"CA.QC\"\n"
                + "    ]\n"
                + "  }\n"
                + "]"
                + "}";

        String regionStr =
            "  {\n"
            + "    \"name\": \"R1\",\n"
            + "    \"countryStateCityList\": [\n"
            + "      \"AG\",\n"
            + "      \"MX.Quintana Roo\",\n"
            + "      \"MX.Solidaridad, Quintana Roo\"\n"
            + "    ]\n"
            + "  }";

        ObjectMapper objectMapper = new ObjectMapper();

        AgentRoutingRegion region = objectMapper.readValue(regionStr, AgentRoutingRegion.class);
        System.out.println(region);

        System.out.println("rawJson:\n" + rawJson);
        AgentRoutingRegionsConfiguration config = objectMapper.readValue(rawJson, AgentRoutingRegionsConfiguration.class);
        System.out.println(config);
        System.out.println(config.regions.get(0).name);
    }
}
