package(default_visibility = ["//visibility:public"])
load("//bazel:main.bzl", "java_package")

java_binary(
    name = "runner",
    main_class = "com.jduan.package1.Main",
    deps = [":package1"],
)

java_library(
    name = "package1",
    srcs = glob(["**/*.java"]),
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "@maven//:com_google_guava_guava",
    ]
)
