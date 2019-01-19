package jduan.delgations

// Delegate most of the methods to "innerSet"
class CoutingSet<T>(
        val innerSet: MutableCollection<T> = HashSet<T>()
) : MutableCollection<T> by innerSet {
    var objectsAdded = 0

    override fun add(element: T): Boolean {
        objectsAdded++
        return innerSet.add(element)
    }

    override fun addAll(c: Collection<T>): Boolean {
        objectsAdded += c.size
        return innerSet.addAll(c)
    }
}

// Why is this better than inheritance?
// The important part is that you aren’t introducing any dependency on how the underlying collection is implemented. For
// example, you don’t care whether that collection implements addAll by calling add in a loop, or if it uses a different
// implementation optimized for a particular case. You have full control over what happens when the client code calls
// your class, and you rely only on the documented API of the underlying collection to implement your operations, so you
// can rely on it continuing to work.

