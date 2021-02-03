package(default_visibility = ["//visibility:public"])
load("//bazel:main.bzl", "java_package")

java_binary(
    name = "package1",
    main_class = "com.jduan.package1.Main",
    runtime_deps = [":package1_lib"],
)

java_library(
    name = "package1_lib",
    srcs = glob(["**/*.java"]),
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "@maven//:com_google_guava_guava",
    ]
)
