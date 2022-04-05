package misc;

import org.jdbi.v3.core.Jdbi;

import java.util.List;

public class JdbiExample {
    public static void main(String[] args) {
        Jdbi jdbi = Jdbi.create("jdbc:hsqldb:mem:test", "sa", "");
        jdbi.useHandle(handle -> {
            handle.execute("CREATE TABLE user (id INTEGER PRIMARY KEY, name VARCHAR(50))");

            // Named parameters
            handle.createUpdate("INSERT INTO user (id, name) VALUES (:id, :name)")
                    .bind("id", 2)
                    .bind("name", "Clarice")
                    .execute();

            List<String> names = handle.createQuery("SELECT name FROM user ORDER BY name")
//                    .mapToBean(User.class)
                    .mapTo(String.class)
                    .list();

            for (String name : names) {
                System.out.println("name: " + name);
            }

        });
    }

    private static class User {
        private String id;
        private String name;

        public User(String id, String name) {
            this.id = id;
            this.name = name;
        }

        @Override
        public String toString() {
            return "User{" +
                    "id='" + id + "', " +
                    "name='" + name + '\'' +
                    '}';
        }
    }
}
