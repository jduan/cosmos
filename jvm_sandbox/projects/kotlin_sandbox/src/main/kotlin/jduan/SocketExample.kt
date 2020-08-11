package jduan

import java.net.ServerSocket

fun main() {
    for (i in 0..10) {
        val ss = ServerSocket(0)
//        ss.reuseAddress = true
//        ss.close()
        println("port: ${ss.localPort}")
//        Thread.sleep(10000)
    }
}
