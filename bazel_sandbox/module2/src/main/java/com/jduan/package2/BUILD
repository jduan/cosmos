package(default_visibility = ["//visibility:public"])
load("//bazel:main.bzl", "java_package")

java_library(
    name = "package2",
    srcs = glob(["*.java"]),
    deps = [
        "@maven//:com_google_guava_guava",
    ]
)
