package jduan

import redis.clients.jedis.JedisPool
import redis.clients.jedis.JedisPoolConfig

fun main() {
    val jedisPoolConfig = JedisPoolConfig()
    val redisHost = "localhost"
    val redisPort = 6379
    val jedisPool = JedisPool(jedisPoolConfig, redisHost, redisPort, 10_000, null)
    jedisPool.resource.use { jedis ->
        val key = "pending_actions"
        val t = jedis.multi()
        val response = t.smembers(key)
        // delete the set entirely
        t.del(key)
        t.exec()

        val actions = response.get()
        actions.forEach { action ->
            println(action)
        }

        // start another multi transaction to remove some keys
        val t2 = jedis.multi()
        listOf("hello2", "world2").forEach { name ->
            t2.sadd(key, name)
        }
        t2.exec()
    }
}
