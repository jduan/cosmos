package concurrency_in_practice.chapter4

import java.util.Collections
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.ConcurrentMap
import javax.annotation.concurrent.Immutable
import javax.annotation.concurrent.ThreadSafe

/**
 * Immutable values can be freely shared and published, so we no longer
 * need to copy the locations when returning them.
 */
@Immutable
class Point(val x: Int, val y: Int)

/**
 * This class doesn't use any explicit synchronization; all access to state
 * is managed by ConcurrentMap, and all the keys and values of the map are immutable.
 */
@ThreadSafe
class DelegatingVehicleTracker(points: Map<String, Point>) {
    private val locations: ConcurrentMap<String, Point>
    private val unmodifiableMap: Map<String, Point>

    init {
        locations = ConcurrentHashMap(points)
        unmodifiableMap = Collections.unmodifiableMap(locations)
    }

    /**
     * Return an unmodifiable but "live" view of the locations. If the underlying
     * locations change, the returned view also reflect the change!
     */
    fun getLocations(): Map<String, Point> = unmodifiableMap

    fun getLocation(id: String): Point? = locations[id]

    fun setLocation(id: String, x: Int, y: Int) {
        if (locations.replace(id, Point(x, y)) == null) {
            throw IllegalArgumentException("Invalid vehicle id: $id")
        }
    }
}
