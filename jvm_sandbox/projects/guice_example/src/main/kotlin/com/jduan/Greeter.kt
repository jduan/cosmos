package com.jduan

import javax.inject.Inject

class Greeter @Inject constructor(@Message val message: String, @Count val count: Int) {
    fun sayHello() {
        repeat(count) {
            println(message)
        }
    }
}
