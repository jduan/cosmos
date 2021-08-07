package com.jduan.weather;

import java.io.InputStream;

import junit.framework.TestCase;
import org.apache.commons.io.IOUtils;

public class WeatherFormatterTest extends TestCase {
    public void testFormat() throws Exception {
        InputStream nyData = getClass().getClassLoader()
                .getResourceAsStream("orinda-weather.json");
        Weather weather = new YahooParser().parse(nyData);
        String formattedResult = new WeatherFormatter().format(weather);
        InputStream expected = getClass().getClassLoader()
                .getResourceAsStream("format-expected.txt");
        assertEquals(IOUtils.toString(expected).trim(), formattedResult.trim());
    }
}
