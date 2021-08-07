package com.jduan.weather;

import org.apache.log4j.Logger;

import java.io.InputStream;
import java.net.URL;
import java.net.URLConnection;

public class YahooRetriever {
    private static Logger log = Logger.getLogger(YahooRetriever.class);

    public InputStream retrieve(String zipcode) throws Exception {
        log.info("Retrieving Weather Data");
        String apikey = System.getenv("API_KEY");
        String url = "http://api.openweathermap.org/data/2.5/weather?zip=" + zipcode + ",us&appid=" + apikey;
        URLConnection conn = new URL(url).openConnection();
        return conn.getInputStream();
    }
}
