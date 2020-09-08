package com.jduan

import com.google.inject.AbstractModule
import com.google.inject.Provides

class DemoModule : AbstractModule() {
    @Provides
    @Count
    fun provideCount(): Int {
        return 3
    }

    @Provides
    @Message
    fun provideMessage(): String {
        return "Hello, world!"
    }
}
