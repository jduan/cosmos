package jduan

import fi.iki.elonen.NanoHTTPD
import org.slf4j.LoggerFactory
import java.util.*

// This server is only for serving health check probes from k8s.
class HttpServer(port: Int) : NanoHTTPD(port) {
    override fun serve(session: IHTTPSession): Response {
        logger.info("Thread name: ${Thread.currentThread().name}")
        if (Random().nextDouble() < 0.3) {
            1 / 0
        }
        logger.info("Serving health check: server is healthy")
        return newFixedLengthResponse(Response.Status.OK, MIME_PLAINTEXT,
            "Server is healthy.\n")
    }

    companion object {
        private val logger = LoggerFactory.getLogger(HttpServer::class.java.name)
    }
}

fun main(args: Array<String>) {
    val httpServer = HttpServer(8080)
    httpServer.start(NanoHTTPD.SOCKET_READ_TIMEOUT, false)
    println("http server started!")
}
