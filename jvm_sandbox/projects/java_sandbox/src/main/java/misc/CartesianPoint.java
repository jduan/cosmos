package misc;

public class CartesianPoint extends Point {
    int x;
    int y;
    public CartesianPoint(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public double distanceTo() {
        return Math.sqrt(x * x + y * y);
    }
}
