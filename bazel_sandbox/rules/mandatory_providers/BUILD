load(":sum.bzl", "sum")

sum(
    name = "sum1",
    number = 1,
    deps = [],
)

sum(
    name = "sum2",
    number = 2,
    deps = [],
)

sum(
    name = "sum3",
    number = 3,
    deps = [":sum1", ":sum2"],
)
