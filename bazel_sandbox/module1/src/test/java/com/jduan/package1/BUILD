package(default_visibility = ["//visibility:public"])
load("//bazel:test.bzl", "java_test_package")

java_library(
    name = "tests",
    testonly = 1,
    srcs = glob(["*Test.java"]),
    deps = [
        "//module1/src/main/java/com/jduan/package1:package1_lib",
        "@maven//:junit_junit",
    ]
)

java_test(
    name = "AllTests",
    srcs = ["AllTests.java"],
    test_class = "com.jduan.package1.AllTests",
    deps = [
        "//testutil",
    ],
    runtime_deps = [
        ":tests",
    ],
)
