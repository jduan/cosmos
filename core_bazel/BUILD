# The general format of load is:
#   load("@//path/to/package:file.bzl", "rule_name")
load(":bazel/archive.bzl", "archive")
load(":bazel/archive2.bzl", "archive2")
load(":bazel/count_items.bzl", "count_items")
load(":bazel/print_count.bzl", "print_count")
load(":bazel/compile.bzl", "compile")
load(":bazel/link.bzl", "link")
load(":bazel/application.bzl", "application")

# archive(
#     name="documentation",
#     files=[
#         "src/README.md",
#     ],
# )

# use genrule (a native rule) to create the target above
genrule(
    name="documentation",
    srcs=[
        "src/README.md",
    ],
    outs=[
        "documentation.zip",
    ],
    cmd="zip $(OUTS) $(SRCS)"
)

archive2(
    name="release",
    files=[
        "src/RELEASE.md",
    ],
    out="release.zip",
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
    hdrs=glob(["src/*.h"]),
)

link(
    name="link",
    objs=[":compile"],
    out="main",
)

application(
    name="main",
    srcs=glob(["src/*.c"]),
    hdrs=glob(["src/*.h"]),
    extra_files=[
        "src/README.md",
        "src/RELEASE.md",
    ]
)

cc_binary(
    name="main2",
    srcs=glob(["src/*.h", "src/*.c"]),
)
