package misc;

import java.util.Iterator;
import java.util.List;

public class ListIterator<E> implements Iterator<E> {
    private List<List<E>> lists;
    private Iterator<List<E>> outerIter;
    private Iterator<E> innerIter;

    public ListIterator(List<List<E>> lists) {
        this.lists = lists;
        outerIter = lists.iterator();
    }

    @Override
    public boolean hasNext() {
        if (innerIter != null && innerIter.hasNext()) {
            return innerIter.hasNext();
        }
        if (outerIter.hasNext()) {
            innerIter = outerIter.next().iterator();
            while (!innerIter.hasNext() && outerIter.hasNext()) {
                innerIter = outerIter.next().iterator();
            }
            return innerIter.hasNext();
        } else {
            return false;
        }
    }

    @Override
    public E next() {
        return innerIter.next();
    }

    public static void main(String[] args) {
        List<List<Integer>> lists = List.of(
            List.of(1, 2, 3),
            List.of(4, 5),
            List.of(),
            List.of(6, 7),
            List.of(8)
        );
        List<List<Integer>> lists2 = List.of(
            List.of(),
            List.of(4, 5),
            List.of(),
            List.of(6, 7),
            List.of(8)
        );
        List<List<Integer>> lists3 = List.of(
            List.of(),
            List.of(),
            List.of(),
            List.of(6, 7),
            List.of()
        );
        ListIterator<Integer> li = new ListIterator<>(lists3);
        while (li.hasNext()) {
            System.out.println(li.next());
        }
    }
}

