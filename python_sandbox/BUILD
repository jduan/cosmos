load("@rules_python//python:defs.bzl", "py_binary")
load("@pip_deps//:requirements.bzl", "requirement")

py_library(
    name = "lib",
    srcs = ["library.py"],
    deps = [
        requirement("pyyaml"),
    ],
)

py_binary(
    name = "yaml_example",
    srcs = ["yaml_example.py"],
    deps = [
        requirement("pyyaml"),
    ],
)

py_binary(
    name = "main",
    srcs = ["main.py"],
    deps = [
        "//:lib",
    ],
)
