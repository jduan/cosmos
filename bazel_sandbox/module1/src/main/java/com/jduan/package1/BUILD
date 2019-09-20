package(default_visibility = ["//visibility:public"])
load("//bazel:main.bzl", "java_package")

java_package(
    name = "package1",
    package = "com.jduan.package1",
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "//third_party:guava",
    ]
)
