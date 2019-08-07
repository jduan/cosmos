package concurrency_in_practice.chapter4

import java.util.Collections
import java.util.Vector
import javax.annotation.concurrent.ThreadSafe

// There are 3 different ways of extending classes with additional thread-safe methods.

/**
 * The synchronization policy of Vector says it uses its "intrinsic lock" so
 * we use the same lock when extending it with the "putIfAbsent" method.
 * This approach can be fragile if the class we are extending doesn't commit to
 * its locking strategy.
 */
@ThreadSafe
class BetterVector<E> : Vector<E>() {
    @Synchronized fun putIfAbsent(x: E): Boolean {
        val absent = !contains(x)
        if (absent) {
            add(x)
        }
        return absent
    }
}

/**
 * Use "client-side locking" to extend List class.
 * "client-side locking" is fragile because it entails putting locking code for class C
 * into classes that are totally unrelated to C. Exercise care when using client-side
 * locking on classes that don't commit to their locking strategy.
 */
@ThreadSafe
class ListHelper<E> {
    val list = Collections.synchronizedList(ArrayList<E>())

    fun putIfAbsent(x: E): Boolean {
        synchronized(list) {
            val absent = !list.contains(x)
            if (absent) {
                list.add(x)
            }
            return absent
        }
    }
}

/**
 * ImprovedList adds an additional level of locking using its own intrinsic lock.
 * It doesn't care whether the underlying List is thread-safe, because it provides its
 * own consistent locking that provides thread safety even if the List is not
 * thread-safe or changes its locking implementation. While the extra layer of
 * synchronization may add some small performance penalty, the implementation is less
 * fragile than attempting to mimic the locking strategy of another object.
 */
@ThreadSafe
class ImprovedList<E>(val list: MutableList<E>) : List<E> {
    @Synchronized
    fun putIfAbsent(x: E): Boolean {
        val absent = !list.contains(x)
        if (absent) {
            list.add(x)
        }
        return absent
    }
    override val size: Int = list.size

    @Synchronized
    override fun contains(element: E): Boolean {
        return list.contains(element)
    }

    @Synchronized
    override fun containsAll(elements: Collection<E>): Boolean {
        return list.containsAll(elements)
    }

    @Synchronized
    override fun get(index: Int): E {
        return list.get(index)
    }

    @Synchronized
    override fun indexOf(element: E): Int {
        return list.indexOf(element)
    }

    @Synchronized
    override fun isEmpty(): Boolean {
        return list.isEmpty()
    }

    @Synchronized
    override fun iterator(): Iterator<E> {
        return list.iterator()
    }

    @Synchronized
    override fun lastIndexOf(element: E): Int {
        return list.lastIndexOf(element)
    }

    @Synchronized
    override fun listIterator(): ListIterator<E> {
        return list.listIterator()
    }

    @Synchronized
    override fun listIterator(index: Int): ListIterator<E> {
        return list.listIterator(index)
    }

    @Synchronized
    override fun subList(fromIndex: Int, toIndex: Int): List<E> {
        return list.subList(fromIndex, toIndex)
    }
}

fun main() {
    val bv = BetterVector<String>()
    bv.putIfAbsent("hello")
    bv.putIfAbsent("world")
    println(bv)

    val listHelper = ListHelper<String>()
    listHelper.list.add("hello")
    listHelper.list.add("world")
    listHelper.putIfAbsent("hello")
    println(listHelper.list)
}
