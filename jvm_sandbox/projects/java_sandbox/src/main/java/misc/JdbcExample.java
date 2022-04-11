package misc;

import java.sql.*;

/**
 * This program prints out the column types of a table.
 */
public class JdbcExample {
    public static void main(String[] args) throws SQLException {
        String username = "jduan@highnoteplatform.com";
        String password = System.getenv("FIREBOLT_PASSWORD");
        try (Connection con = DriverManager.getConnection(
                "jdbc:firebolt://api.app.firebolt.io/FastSearchPOC", username, password)) {
            try (Statement stmt = con.createStatement()) {
                String tableSql = "Select * from tx_ledger_entry where 1<0";
                long startTime = System.nanoTime();
                ResultSet rs = stmt.executeQuery(tableSql);
                ResultSetMetaData metaData = rs.getMetaData();
                for (int c = 1; c <= metaData.getColumnCount(); c++) {
                    String name = metaData.getColumnName(c);
                    String typeName = metaData.getColumnTypeName(c);
                    System.out.println(String.format("column name %s, column type %s", name, typeName));
                }
                long endTime = System.nanoTime();
                double duration = (endTime - startTime) / 1000000;
                System.out.println("query took " + duration + "ms");
//                    System.out.println(rs);
            }
        }
    }
}
