package jduan

import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.sse.EventSource
import okhttp3.sse.EventSourceListener
import okhttp3.sse.EventSources
import java.security.SecureRandom
import java.security.cert.X509Certificate
import java.util.concurrent.TimeUnit
import java.util.logging.Level
import javax.net.ssl.SSLContext
import javax.net.ssl.SSLSocketFactory
import javax.net.ssl.X509TrustManager


object PassthroughTrustManager : X509TrustManager {
    override fun checkClientTrusted(p0: Array<out X509Certificate>?, p1: String?) = Unit

    override fun checkServerTrusted(p0: Array<out X509Certificate>?, p1: String?) = Unit

    override fun getAcceptedIssuers() = arrayOf<X509Certificate>()
}

object FakeSslContext {
    val trustManager by lazy { PassthroughTrustManager }

    val sslSocketFactory: SSLSocketFactory by lazy {
        val sslContext = SSLContext.getInstance("SSL").also {
            it.init(null, arrayOf(trustManager), SecureRandom())
        }
        sslContext.socketFactory
    }
}

fun testGet() {
    val client: OkHttpClient = OkHttpClient.Builder()
        .readTimeout(0, TimeUnit.SECONDS)
        .sslSocketFactory(FakeSslContext.sslSocketFactory, FakeSslContext.trustManager)
//        .addInterceptor(HttpLoggingInterceptor().setLevel(HttpLoggingInterceptor.Level.BASIC))
        .build()
    val eventSourceFactory: EventSource.Factory = EventSources.createFactory(client)
    val url = "https://gradle.synapse:4848/build-export/v1/build/67k2k5dsxqsmq/events?eventTypes=UserTag"
    val request = Request.Builder().url(url).build()
    val eventFilter = "Build"
    val listener = object : EventSourceListener() {
        override fun onEvent(eventSource: EventSource, id: String?, type: String?, data: String) {
            println("onEvent: $data")
        }

        override fun onOpen(eventSource: EventSource, response: Response) {
            println("onOpen, url=${eventSource.request().url()}")
        }

        override fun onClosed(eventSource: EventSource) {
            println("onClosed, url=${eventSource.request().url()}")
        }

        override fun onFailure(eventSource: EventSource?, t: Throwable?, response: Response?) {
            t!!.printStackTrace()
        }
    }
    eventSourceFactory.newEventSource(request, listener)
}
