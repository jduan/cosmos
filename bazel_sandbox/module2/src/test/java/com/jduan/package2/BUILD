package(default_visibility = ["//visibility:public"])
load("//bazel:test.bzl", "java_test_package")

java_test_package(
    name = "package2",
    package = "com.jduan.package2",
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "@maven//:com_google_guava_guava",
        "@maven//:junit_jhunit",
    ]
)
