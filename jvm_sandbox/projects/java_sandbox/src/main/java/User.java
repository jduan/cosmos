import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.ToString;

@Getter
@Setter
@ToString
@NoArgsConstructor
public class User {
    private String firstName;
    private String lastName;
    private int age;

    public static void main(String[] args) {
        User user = new User();
        user.setFirstName("Jack");
        user.setLastName("Dorsey");
        user.setAge(40);
        System.out.println(user);
    }
}
