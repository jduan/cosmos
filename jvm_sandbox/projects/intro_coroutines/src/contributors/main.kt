package contributors

import java.io.File

fun main() {
    val file = File("/tmp/1")
    file.walk().forEach { println("entry: $it") }
//    setDefaultFontSize(18f)
//    ContributorsUI().apply {
//        pack()
//        setLocationRelativeTo(null)
//        isVisible = true
//    }
}
