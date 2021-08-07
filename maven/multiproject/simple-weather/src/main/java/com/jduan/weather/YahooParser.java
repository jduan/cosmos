package com.jduan.weather;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.log4j.Logger;

import java.io.InputStream;

public class YahooParser {
    private static Logger log = Logger.getLogger(YahooParser.class);
    private ObjectMapper objectMapper = new ObjectMapper();

    public Weather parse(InputStream inputStream) throws Exception {
        Weather weather = new Weather();

        log.info("Parsing JSON Response");
        JsonNode jsonNode = objectMapper.readTree(inputStream);
        weather.setTemp(jsonNode.get("main").get("temp").asText());
        weather.setHumidity(jsonNode.get("main").get("humidity").asText());

        return weather;
    }
}
