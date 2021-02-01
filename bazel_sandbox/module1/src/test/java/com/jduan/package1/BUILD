package(default_visibility = ["//visibility:public"])
load("//bazel:test.bzl", "java_test_package")

java_test(
    name = "package1",
    deps = [
        "//module1/src/main/java/com/jduan/package1:package1",
        "@maven//:com_google_guava_guava",
        "@maven//:junit_junit",
    ]
)
