package(default_visibility = ["//visibility:public"])
# load("//bazel:test.bzl", "java_test_package")

java_library(
    name = "tests",
    testonly = 1,
    srcs = glob(["*.java"]),
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "//testutil",
        "@maven//:junit_junit",
    ]
)

java_test(
    name = "AllTests",
    test_class = "com.jduan.package2.AllTests",
    runtime_deps = [
        ":tests",
        "//testutil",
        "@maven//:junit_junit",
    ],
)
