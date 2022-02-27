package misc;

import java.sql.*;

public class FireboltExample {
    public static void main(String[] args) throws SQLException {
        String username = "jduan@highnoteplatform.com";
        String password = System.getenv("FIREBOLT_PASSWORD");
        try (Connection con = DriverManager.getConnection(
                "jdbc:firebolt://api.app.firebolt.io/FastSearchPOC", username, password)) {
            try (Statement stmt = con.createStatement()) {
                for (int i = 0; i < 10; i++) {
                    String tableSql = "Select account_type,change_amount,financial_event_type,impact,memo "
                            + "from ledger_events where account_type = 'PREPAID_CARD_ACCOUNT2'";
                    long startTime = System.nanoTime();
                    ResultSet rs = stmt.executeQuery(tableSql);
                    long endTime = System.nanoTime();
                    double duration = (endTime - startTime) / 1000000;
                    System.out.println("query took " + duration + "ms");
//                    System.out.println(rs);
                }
            }
        }
    }
}
