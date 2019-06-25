package jduan

fun main() {
//  val PAIR_REGEX = "\\(.*?\\)".toRegex()
  val PAIR_REGEX = "\\(.*?, .*?\\)".toRegex()
  println(PAIR_REGEX)
  val content = "(Git SHA, 2263ace0c473161022080fed596c73bae6103463), (Git branch, refs/heads/master), (Hostname, 7e871e9a6a5c), (shadow.:projects:trips:matcha:shadowJar.dependencies, 567), (shadow.:projects:trips:matcha:shadowJar.relocations, ), (shadow.:projects:trips:matcha:shadowJar.configurations, runtimeClasspath)"

  println("all matches: ${PAIR_REGEX.findAll(content).toList().size}")
  val customValues = PAIR_REGEX.findAll(content)
    .map {
      println(it.groupValues)
      val (first, second) = it.value.split(", ")
      first.drop(1) to second.take(second.length - 1)
    }
    .toMap()
  println(customValues)
}
