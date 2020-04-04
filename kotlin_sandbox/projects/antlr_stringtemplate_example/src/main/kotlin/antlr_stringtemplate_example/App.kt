package antlr_stringtemplate_example

import org.stringtemplate.v4.ST
import org.stringtemplate.v4.STGroupDir
import org.stringtemplate.v4.STGroupFile

class App {
    val greeting: String
        get() {
            return "Hello world."
        }
}

fun helloWorld() {
    val hello = ST("Hello, <name>")
    hello.add("name", "World!")
    println(hello.render())
}

// Load separate templates from a directory.
fun templateFile() {
    // "string_templates" are under "src/main/resources", which ST searches for.
    val group = STGroupDir("string_templates")
    val decl = group.getInstanceOf("decl")
    decl.add("type", "int")
    decl.add("name", "x")
    decl.add("value", 0)
    println("decl renders: ${decl.render()}")
}

// Load a single group file that has multiple templates.
fun groupFile() {
    val group = STGroupFile("string_templates/group_files/test.stg")
    val decl = group.getInstanceOf("decl")
    decl.add("type", "int")
    decl.add("name", "x")
    decl.add("value", 0)
    println("decl renders: ${decl.render()}")
}

fun main(args: Array<String>) {
    helloWorld()
    templateFile()
    groupFile()
    println(App().greeting)
}
