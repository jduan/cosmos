package misc;

import org.jdbi.v3.core.Jdbi;
import org.jdbi.v3.core.generic.GenericType;
import org.jdbi.v3.core.result.ResultIterable;
import org.jdbi.v3.core.result.ResultProducers;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class JdbiExample {
    public static void main(String[] args) {
        Jdbi jdbi = Jdbi.create("jdbc:hsqldb:mem:test", "sa", "");
        jdbi.useHandle(handle -> {
            handle.execute("CREATE TABLE user (id INTEGER PRIMARY KEY, name VARCHAR(50))");

            // Named parameters
            handle.createUpdate("INSERT INTO user (id, name) VALUES (:id, :name)")
                    .bind("id", 1)
                    .bind("name", "John")
                    .execute();
            handle.createUpdate("INSERT INTO user (id, name) VALUES (:id, :name)")
                    .bind("id", 2)
                    .bind("name", "Ryan")
                    .execute();

            List<String> names = handle.createQuery("SELECT name FROM user ORDER BY name")
//                    .mapToBean(User.class)
                    .mapTo(String.class)
                    .list();

            List<String> names2 = handle.createQuery("SELECT name FROM user ORDER BY name")
                    .execute(ResultProducers.returningResults())
                    .collectInto(new GenericType<List<String>>() {});

            ResultIterable<Map<String, Object>> names3 = handle.createQuery("SELECT id, name FROM user ORDER BY name")
                    .execute(ResultProducers.returningResults())
                    .mapToMap();
            List<Map<String, Object>> names4 = names3.stream().collect(Collectors.toList());


            for (Map<String, Object> map : names4) {
                for (String key : map.keySet()) {
                    System.out.println(String.format("key: %s, value: %s", key, map.get(key)));
                }
                System.out.println();
            }

//            for (String name : names2) {
//                System.out.println("name: " + name);
//            }

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
