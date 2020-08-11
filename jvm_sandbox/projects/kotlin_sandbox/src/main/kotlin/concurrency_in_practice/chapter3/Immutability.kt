package concurrency_in_practice.chapter3

import java.math.BigInteger
import java.util.Arrays
import javax.annotation.concurrent.Immutable
import javax.annotation.concurrent.ThreadSafe
import javax.servlet.Servlet
import javax.servlet.ServletConfig
import javax.servlet.ServletRequest
import javax.servlet.ServletResponse

/**
 * This demos that immutable objects can still use mutable objects internally to
 * manage their state as long as there's no way to access those internal, mutable objects
 * from the outside.
 */
@Immutable
class ThreeStooges {
    private val stooges = HashSet<String>()

    init {
        stooges.add("Moe")
        stooges.add("Larry")
        stooges.add("Curly")
    }

    fun isStooge(name: String) = stooges.contains(name)
}

/**
 * This class is immutable because there's no way to change its 2 private state variables.
 */
@Immutable
class OneValueCache(private val lastNumber: BigInteger, lastFactors: Array<BigInteger>) {
    private val lastFactors: Array<BigInteger> = Arrays.copyOf(lastFactors, lastFactors.size)

    fun getFactors(bi: BigInteger): Array<BigInteger>? {
        if (lastNumber == bi) {
            return null
        } else {
            return Arrays.copyOf(lastFactors, lastFactors.size)
        }
    }
}

@ThreadSafe
class VolatileCachedFactorizer : Servlet {
    @Volatile
    private var cache: OneValueCache? = null

    override fun destroy() {
        TODO("not implemented")
    }

    override fun init(config: ServletConfig?) {
        TODO("not implemented")
    }

    override fun getServletInfo(): String {
        TODO("not implemented")
    }

    override fun service(req: ServletRequest?, res: ServletResponse?) {
        // Extract BigInteger from req
        val bi = BigInteger("123")
        if (cache != null) {
            val factors = cache?.getFactors(bi)
            if (factors != null) {
                // use the factors
            } else {
                // calculate factors
                val factors = arrayOf(BigInteger("12"))
                cache = OneValueCache(bi, factors)
            }
        } else {
            // calculate factors
            val factors = arrayOf(BigInteger("12"))
            cache = OneValueCache(bi, factors)
        }
    }

    override fun getServletConfig(): ServletConfig {
        TODO("not implemented")
    }

}


fun main() {
    val threeStooges = ThreeStooges()
    println(threeStooges.isStooge("Moe"))
    println(threeStooges.isStooge("John"))
}
