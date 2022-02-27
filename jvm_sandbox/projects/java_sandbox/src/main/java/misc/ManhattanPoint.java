package misc;

public class ManhattanPoint extends Point {
    int x;
    int y;
    public ManhattanPoint(int x, int y) {
        this.x = x;
        this.y = y;
    }
    @Override
    public double distanceTo() {
        return Math.abs(x) + Math.abs(y);
    }
}
