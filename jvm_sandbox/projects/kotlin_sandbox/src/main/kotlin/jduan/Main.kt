package jduan

import jduan.collections.joinToString
import java.io.File
import java.nio.file.Files

fun main() {
//    val label = getLabelSchema(File("schemas/_xrepo/briefcase/build/classes/java/thriftSrcReviewReportingData"))
    val label = getLabelSchema(File("schemas/_xrepo/winston/build/classes/java/thriftClient"))
//    val label = getLabelSchema(File("schemas/request_context/build/classes/java/thriftSrcRequestContextData"))
//    val label = getLabelSchema(File("schemas/athena/build/classes/java/thriftSrcCommonAthenaCommonData"))
    println("label: $label")
}

// Given classesDir looks like this:
//  * schemas/athena/build/classes/java/thriftSrcCommonAthenaCommonData
//  * or schemas/_xrepo/winston/build/classes/java/thriftClient
// return a bazel label: //schemas/athena:athena_client
fun getLabelSchema(classesDir: File): String {
    println("getLabelSchema classesDir: ${classesDir.path}")
    val parts = classesDir.path.split("/").toMutableList()
    var schemaName = parts[1]
    val isXrepo = if (schemaName == "_xrepo") {
        parts.removeAt(1)
        schemaName = parts[1]
        true
    } else {
        false
    }
    val schemaDir = File("schemas/$schemaName")
    val dataName = parts[5]
    val (path, target) = if (dataName == "thriftClient") {
        Pair("", "${schemaName}_client")
    } else {
        val schemaSrcDir = if (isXrepo) {
            File("${parts[0]}/_xrepo/${parts[1]}")
        } else {
            File("${parts[0]}/${parts[1]}")
        }
        // find the thrift file that generated these classes
        val thriftFile = schemaSrcDir.walk().onEnter { !Files.isSymbolicLink(it.toPath()) }
                .filter { it.name.endsWith("thrift") }
                .apply { println(this.toList()) }
                .first {
                    val items = it.path.replace("_xrepo/", "").replace(".thrift", "").split("/")
                    val rest = items.subList(2, items.size).toMutableList()
                    val newLast = capitalizeString(rest.last())
                    rest[rest.size - 1] = newLast
                    val newRest = rest.map { capitalizeString(it) }
                    println("newRest: $newRest")
                    newRest.joinToString(separator = "", prefix = "", suffix = "") == dataName.substring("thrift".length)
                }
        println(thriftFile)
        val parentDir = if (thriftFile.parentFile.name == "src") {
            thriftFile.parentFile.parentFile
        } else {
            thriftFile.parentFile
        }
        val path = parentDir.relativeTo(schemaDir).path
        val target = thriftFile.name.replace(".thrift", "")
        Pair(path, target)
    }
    return if (isXrepo) {
        "@xrepo_$schemaName//:$target"
    } else {
        if (path.isEmpty()) {
            "//schemas/$schemaName:$target"
        } else {
            "//schemas/$schemaName/$path:$target"
        }
    }
}

private fun capitalizeString(str: String): String {
    return str.split("_").map { it.capitalize() }.joinToString(separator = "", prefix = "", suffix = "")
}

// This shows that "return" still works even if you have a "finally" block
fun hello(): Int {
    try {
        println("hello")
        return 1
    } finally {
        println("world")
    }
}

fun parseNumber(maybeNumber: String) {
    val number = try {
        Integer.parseInt(maybeNumber)
    } catch (e: NumberFormatException) {
        return
    }

    println(number)
}
