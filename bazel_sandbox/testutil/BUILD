package(default_visibility = ["//visibility:public"])
load("//bazel:main.bzl", "java_package")

java_library(
    name = "testutil",
    srcs = glob(["src/main/java/com/jduan/testutil/*.java"]),
    deps = [
        "@maven//:com_google_guava_guava",
        "@maven//:junit_junit",
    ],
)
