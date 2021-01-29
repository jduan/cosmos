load(":sum.bzl", "sum")
load(":empty.bzl", "empty")

sum(
    name = "sum1",
    number = 1,
    deps = [],
)

empty(
    name = "empty",
)

sum(
    name = "sum3",
    number = 3,
    deps = [":sum1", ":empty"],
)
