package(default_visibility = ["//visibility:public"])
# load("//bazel:test.bzl", "java_test_package")

java_library(
    name = "tests",
    testonly = 1,
    srcs = glob(["*Test.java"]),
    deps = [
        "//module2/src/main/java/com/jduan/package2:package2",
        "@maven//:junit_junit",
    ]
)

java_test(
    name = "AllTests",
    srcs = ["AllTests.java"],
    test_class = "com.jduan.package2.AllTests",
    deps = [
        "//testutil",
    ],
    runtime_deps = [
        ":tests",
    ],
)
