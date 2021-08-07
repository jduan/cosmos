package com.jduan.weather;

import junit.framework.TestCase;

import java.io.InputStream;

public class YahooParserTest extends TestCase {
    public YahooParserTest(String name) {
        super(name);
    }

    public void testParser() throws Exception {
        InputStream nyData = getClass().getClassLoader()
                .getResourceAsStream("orinda-weather.json");
        Weather weather = new YahooParser().parse(nyData);
        assertEquals("300.12", weather.getTemp());
        assertEquals("46", weather.getHumidity());
    }
}
