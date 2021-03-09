package jduan

import java.io.File
import java.nio.file.Files
import java.nio.file.StandardCopyOption
import java.util.concurrent.TimeUnit

typealias JavaPackage = String

class JdepsException(message: String) : Exception(message)

fun main() {
//    JarParser.extractPackages(File("/Users/jingjing_duan/.gradle/caches/modules-2/files-2.1/org.apache.hbase/hbase-server/0.99.0/c66f6999340e904b29263d61d4c2bdacd8701bc3/hbase-server-0.99.0.jar"))
    println(JarParser.extractJdepsData(File("/Users/jingjing_duan/repos2/treehouse_worktree3/common/utils"),
            File("/Users/jingjing_duan/repos2/treehouse_worktree3/common/utils/functions/build/libs/functions.jar")))
}

object JarParser {
    /**
     * Given a jar file, it runs "jdeps" on the jar, parses the jdeps output, and returns a list
     * of "java packages" that belong to this jar.
     */
    fun extractPackages(jarFile: File) {
        println("-------------- JarPaser processing $jarFile")
        val tmpdir = createTempDir()
        val newFile = tmpdir.resolve(jarFile.name)
        Files.copy(jarFile.toPath(), newFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
        val process = ProcessBuilder("jar", "-tvf", newFile.path)
                .redirectOutput(ProcessBuilder.Redirect.PIPE)
                .redirectError(ProcessBuilder.Redirect.PIPE)
                .start()
//        if (!process.waitFor(60, TimeUnit.SECONDS)) {
//            process.destroy()
//            throw RuntimeException("execution timed out: $process")
//        }
//        if (process.exitValue() != 0) {
//            throw RuntimeException("execution failed with code ${process.exitValue()}: ${process.errorStream.bufferedReader().readLines()}")
//        }
        val output = process.inputStream.bufferedReader().readText()
        val packages = parseJarOutput(output)
        println("packages: $packages")
    }

    /**
     * Use this method on internal project jars.
     *
     * See the documentation for DotFileData
     */
    fun extractJdepsData(projectPath: File, jarFile: File): DotFileData? {
        val dotFile = generateDotFile(jarFile)
        return if (dotFile == null) {
            null
        } else {
            DotFileParser.parse(projectPath, dotFile)
        }
    }

    @Throws(JdepsException::class)
    private fun generateDotFile(jarFile: File): File? {
        println("-------------- JdepsHelper processing $jarFile")
        val tmpdir = createTempDir()
        val newFile = tmpdir.resolve(jarFile.name)
        Files.copy(jarFile.toPath(), newFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
        val process = ProcessBuilder("jdeps", "--multi-release", "base", "-dotoutput", tmpdir.path, newFile.path)
                .redirectOutput(ProcessBuilder.Redirect.PIPE)
                .redirectError(ProcessBuilder.Redirect.PIPE)
                .start()
        if (!process.waitFor(30, TimeUnit.SECONDS)) {
            process.destroy()
            throw RuntimeException("execution timed out: $process")
        }
        if (process.exitValue() != 0) {
            throw JdepsException("execution failed with code ${process.exitValue()}: ${process.errorStream.bufferedReader().readLines()}")
        }

        return tmpdir.listFiles().firstOrNull { dotFile ->
            dotFile.name.endsWith(".dot") && dotFile.name != "summary.dot"
        }
    }

    private fun parseJarOutput(output: String): Set<JavaPackage> {
        return output.split("\n").mapNotNull { line ->
            val lastColumn = line.split(" ").last()
            if (lastColumn.endsWith(".class")) {
                val items = lastColumn.split("/")
                items.subList(0, items.size - 1).joinToString(separator = ".")
            } else {
                null
            }
        }.toSet()
    }
}

/**
 * This data class contains data extracted from a jar:
 *
 * javaPackages: the java packages that are inside this jar, such as "com.airbnb.common.services.utils"
 *
 * packageToBazelPath maps "java packages" to the future bazel path, such as
 *  "com.airbnb.common.services.utils" -> "common/service-framework/src/main/java/com/airbnb/common/services/utils/"
 *
 * bazelPathDeps maps "bazel path" to a list of its deps, such as
 *  "common/service-framework/src/main/java/com/airbnb/common/services/utils/" -> [
 *      "com.airbnb.common.context.api.restore",   // internal dependency
 *      "com.google.common.collect",               // external dependency
 *  ]
 */
data class DotFileData(
        val javaPackages: Set<JavaPackage>,
        val packageToBazelPath: Map<JavaPackage, String>,
        val bazelPathDeps: Map<String, List<JavaPackage>>
) {
    // Return the set of external dependencies of this jar
    fun getExternalDeps(): Set<JavaPackage> {
        return bazelPathDeps.values.flatMap { pkgs ->
            pkgs.filterNot { pkg ->
                pkg.startsWith("com.airbnb")
            }
        }.toSet()
    }
}

/**
 * Dot files look like this:
 *
digraph "service-framework-0.1.0.jar" {
// Path: /Users/jingjing_duan/repos2/treehouse_worktree3/common/service-framework/build/libs/service-framework-0.1.0.jar
"com.airbnb.common.services"                       -> "com.airbnb.common.context.api.restore (not found)";
"com.airbnb.common.services"                       -> "com.google.common.collect (not found)";
"com.airbnb.common.services"                       -> "java.lang";
"com.airbnb.common.services"                       -> "java.lang.invoke";
"com.airbnb.common.services"                       -> "java.util";
"com.airbnb.common.services"                       -> "java.util.concurrent";
"com.airbnb.common.services"                       -> "java.util.function";
"com.airbnb.common.services"                       -> "org.slf4j (not found)";
"com.airbnb.common.services.utils"                 -> "com.airbnb.common.miniprogram (not found)";
}
 */
object DotFileParser {
    fun parse(projectPath: File, dotFile: File): DotFileData {
        val map1 = mutableMapOf<String, String>()
        val map2 = mutableMapOf<String, MutableList<String>>()
        dotFile.readLines().forEach { line ->
            val match = dotFileRegex.find(line)
            if (match != null) {
                val javaPackage = match.groupValues[1]
                val dependency = match.groupValues[2].replace(" (.*)".toRegex(), "")
                val packagePath = projectPath.resolve("src/main/java").resolve(javaPackage.replace('.', '/')).path
                map1[javaPackage] = packagePath
                if (dependency.startsWith("java.")) {
                    return@forEach
                }
                val deps = map2.getOrDefault(packagePath, mutableListOf())
                deps.add(dependency)
                map2[packagePath] = deps
            }
        }

        return DotFileData(map1.keys, map1, map2)
    }

    fun parse(dotFile: File): Set<JavaPackage> {
        return dotFile.readLines().mapNotNull { line ->
            val match = dotFileRegex.find(line)
            if (match != null) {
                val javaPackage = match.groupValues[1]
                javaPackage
            } else {
                null
            }
        }.toSet()
    }

    private val dotFileRegex = "\"(.*)\"\\s*-> \"(.*)\"".toRegex()
}
