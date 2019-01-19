package rxjava;

import io.reactivex.Flowable;

public class RxExample {
    public static void main(String... args) {
        Flowable.fromArray(args).subscribe(s -> System.out.println("hello " + s + "!"));
    }
}
