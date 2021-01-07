package jackson;

import java.util.List;

public class View {

    private List<Shape> shapes;

    public List<Shape> getShapes() {
        return shapes;
    }

    public void setShapes(List<Shape> shapes) {
        this.shapes = shapes;
    }

    @Override
    public String toString() {
        return "View{" +
            "shapes=" + shapes +
            '}';
    }
}
