# The general format of load is:
#   load("@//path/to/package:file.bzl", "rule_name")
load(":bazel/archive.bzl", "archive")
load(":bazel/count_items.bzl", "count_items")
load(":bazel/print_count.bzl", "print_count")
load(":bazel/compile.bzl", "compile")
load(":bazel/link.bzl", "link")

archive(
    name="documentation",
    files=[
        "src/README.md",
    ],
)

archive(
    name="release",
    files=[
        "src/RELEASE.md",
    ],
)

archive(
    name="all",
    files=[
        "src/README.md",
        "src/RELEASE.md",
    ],
)

count_items(
    name="count",
    items=[1,7,13,33],
)

print_count(
    name="print",
    rule_dep=":count",
)

compile(
    name="compile",
    srcs=glob(["src/*.c"]),
)

link(
    name="link",
    objs=[":compile"],
    out="main",
)
