package concurrency_in_practice.chapter4

import java.util.Collections
import javax.annotation.concurrent.GuardedBy
import javax.annotation.concurrent.NotThreadSafe
import javax.annotation.concurrent.ThreadSafe

@NotThreadSafe
class MutablePoint(var x: Int, var y: Int) {
    constructor() : this(0, 0)
    constructor(mp: MutablePoint) : this(mp.x, mp.y)
    override fun toString(): String = "MutablePoint($x, $y)"
}

/**
 * Even though MutablePoint is not thread-safe, the tracker class is.
 * Neither the map nor any of the mutable points it contains is ever published.
 * When we need to return vehicle locations to callers, the appropriate values
 * are copied using either the MutablePoint's copy constructor or "deepCopy".
 */
@ThreadSafe
class MonitorVehicleTracker(locations: Map<String, MutablePoint>) {
    @GuardedBy("this")
    private val locations: Map<String, MutablePoint>

    init {
        this.locations = deepCopy(locations)
    }

    /**
     * Return a snapshot of the locations. The returned map doesn't change even
     * if the underlying locations change in the future. This may be a problem
     * or a feature!
     */
    @Synchronized
    fun getLocations(): Map<String, MutablePoint> = deepCopy(locations)

    @Synchronized
    fun getLocation(id: String): MutablePoint? {
        val location = locations[id]
        return if (location != null) {
            MutablePoint(location)
        } else {
            null
        }
    }

    @Synchronized
    fun setLocation(id: String, x: Int, y: Int) {
        val location = locations[id]
        if (location != null) {
            location.x = x
            location.y = y
        } else {
            throw IllegalArgumentException("No such id: $id")
        }
    }

    companion object {
        private fun deepCopy(m: Map<String, MutablePoint>): Map<String, MutablePoint> {
            val result = HashMap<String, MutablePoint>()
            for ((key, value) in m.entries) {
                result[key] = MutablePoint(value)
            }
            // the map is read-only but values can be changed because MutablePoint
            // is mutable!
            return Collections.unmodifiableMap(result)
        }
    }
}

fun main() {
    val locations = mapOf(
        "Beijing" to MutablePoint(1, 2),
        "Tianjing" to MutablePoint(3, 4)
    )
    val tracker = MonitorVehicleTracker(locations)
    println("tracker: ${tracker.getLocations()}")
    tracker.setLocation("Beijing", 10, 20)
    println("tracker: ${tracker.getLocations()}")
}
