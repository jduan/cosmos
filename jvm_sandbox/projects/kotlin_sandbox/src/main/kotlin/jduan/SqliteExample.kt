package jduan

import java.sql.Connection
import java.sql.DriverManager
import java.sql.SQLException


object Connect {
    /**
     * Connect to a sample database
     */
    fun connect(): Connection {
        // db parameters
        val url = "jdbc:sqlite:/Users/jingjing_duan/dropwizard_upgrade/sqlite3.db"
        // create a connection to the database
        val conn = DriverManager.getConnection(url)
        println("Connection to SQLite has been established.")
        return conn
    }

    // For some reason, this insert fails for a brand new table without any data yet.
    // If you manually insert some data to the table, the query would work!
    fun insert() {
        val sql = "INSERT INTO arborist(scan_id, classname, testname, failure) VALUES(?,?,?,?)"

        try {
            val conn = connect()
            println("going to prepare statement")
            val pstmt = conn.prepareStatement(sql)
            println("statement prepared")
            pstmt.setString(1, "abc")
            pstmt.setString(2, "com.airbnb.MyTest")
            pstmt.setString(3, "testAbc")
            pstmt.setString(4, "OOM")
            pstmt.executeUpdate()
            println("inserted a row to sqlite")
        } catch (e: SQLException) {
            println("SQLException: ${e.message}")
        }
    }

    fun query() {
        val sql = "SELECT * FROM arborist"

        try {
            val conn = connect()
            val stmt = conn.createStatement()
            val result = stmt.executeQuery(sql)
            while (result.next()) {
                println("scan id: ${result.getString("scan_id")}")
            }

        } catch (e: SQLException) {
            println("SQLException: ${e.message}")
        }
    }

    /**
     * @param args the command line arguments
     */
    @JvmStatic
    fun main(args: Array<String>) {
        insert()
    }
}
