package org.sonatype.mavenbook.web;

import com.jduan.weather.WeatherService;

import java.io.*;
import javax.servlet.*;
import javax.servlet.http.*;

public class SimpleServlet extends HttpServlet {
    public void doGet(HttpServletRequest request,
                      HttpServletResponse response)
            throws ServletException, IOException {
        String zip = request.getParameter("zip");
        WeatherService service = new WeatherService();
        PrintWriter out = response.getWriter();
        try {
            out.println(service.retrieveForecast(zip));
        } catch (Exception e) {
            out.println("Error retrieving forecast: " + e.getMessage());
        }
        out.flush();
        out.close();
    }
}