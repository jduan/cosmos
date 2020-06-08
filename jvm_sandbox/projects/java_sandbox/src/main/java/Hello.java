public class Hello {
    private Object lockObject = new Object();
    public String getName() {
        synchronized (lockObject) {
            String firstName = getFirstName();
            String lastName = getLastName();
            return firstName + " " + lastName;
        }
    }

    private String getLastName() {
        synchronized (lockObject) {
            return "John";
        }
    }

    private String getFirstName() {
        synchronized (lockObject) {
            return "Davis";
        }
    }

    public String hello() {
        return "world";
    }

    public static void main(String[] args) {
        Hello hello = new Hello();
        System.out.println(hello.getName());
    }
}
