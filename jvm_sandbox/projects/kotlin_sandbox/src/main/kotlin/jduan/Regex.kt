package jduan

fun main() {
    val PAIR_REGEX = "\\(.*?, .*?\\)".toRegex()
    println(PAIR_REGEX)
    val content = "(Git SHA, 2263ace0c473161022080fed596c73bae6103463), (Git branch, refs/heads/master), (Hostname, 7e871e9a6a5c), (shadow.:projects:trips:matcha:shadowJar.dependencies, 567), (shadow.:projects:trips:matcha:shadowJar.relocations, ), (shadow.:projects:trips:matcha:shadowJar.configurations, runtimeClasspath)"

    println("all matches: ${PAIR_REGEX.findAll(content).toList().size}")
    // findAll returns an iterator of all the matches.
    // For each match, groupValues returns a list where the first match starts
    // at index 1. Index 0 is for the whole segment.
    val customValues = PAIR_REGEX.findAll(content)
        .map {
            println(it.groupValues)
            val (first, second) = it.value.split(", ")
            first.drop(1) to second.take(second.length - 1)
        }
        .toMap()
    println(customValues)
}
