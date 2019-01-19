package jduan.interfaces

interface Clickable {
    fun click()

    // method with default implementation
    fun showOff() = println("I'm focusable!")
}

interface Focusable {
    fun setFocus(b: Boolean) =
            println("I ${if (b) "got" else "lost"} focus.")
    fun showOff() = println("I'm focusable too!")
}

// : means extends and/or implements
class Button : Clickable, Focusable {
    // "override" is mandatory
    override fun click() {
        println("I was clicked")
    }

    override fun showOff() {
        super<Clickable>.showOff()
        super<Focusable>.showOff()
    }
}

// This class is open; others can inherit from it
open class RichButton : Clickable {
    // This method is final; you can't override it in a subclass
    fun disable() {}

    // This method is open; you may override it in a subclass
    open fun animate() {}

    // Overridden methods are open by default
    // You can add "final" to forbid overriding
    override fun click() {}
}

// Abstract classes can't be instantiated
abstract class Animated {
    // abstract functions are always open; you don't need an explicit "open" modifier
    abstract fun animate()

    // non-abstract functions aren't open by default but you can make them open
    open fun stopAnimating() {}

    fun animateTwice() {}
}

internal open class TalkativeButton : Focusable {
    private fun yell() = println("Hey!")

    protected fun whisper() = println("Let's talk!")
}

// The following function violates a bunch of rules.
//fun TalkativeButton.giveSpeech() {
//    yell()
//    whisper()
//}

open class User(val nickname: String,
           val isSubscribed: Boolean = true)

class TwitterUser(nickname: String) : User(nickname) {
    // secondary constructor
    constructor(firstname: String, lastname: String) : this(firstname + lastname) {

    }
}

// This means Secretive can't be instantiated!
class Secretive private constructor() {}