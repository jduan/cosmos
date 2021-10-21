load(":bazel/archive.bzl", "archive")

archive(
    name="documentation",
    files=[
        "src/README.md",
    ],
    out="documentation.zip",
)

archive(
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
    out="all.zip",
)
