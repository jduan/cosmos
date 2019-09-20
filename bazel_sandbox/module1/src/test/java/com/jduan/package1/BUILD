package(default_visibility = ["//visibility:public"])
load("//bazel:test.bzl", "java_test_package")

java_test_package(
    name = "package1",
    package = "com.jduan.package1",
    deps = [
        "//module1/src/main/java/com/jduan/package1:package1",
        "//third_party:guava",
        "//third_party:junit",
    ]
)
