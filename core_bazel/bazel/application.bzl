load(":bazel/archive.bzl", "archive")
load(":bazel/compile.bzl", "compile")
load(":bazel/link.bzl", "link")

# Macros are pure functions that invoke rules to create targets.
def application(name, srcs, hdrs = [], extra_files = []):
    compile(
        name = "compile",
        srcs = srcs,
        hdrs = hdrs,
    )

    link(
        name = "link",
        objs = [":compile"],
        out = "main",
    )

    archive_files = [":link"]
    archive_files.extend(extra_files)

    archive(
        name = name,
        files = archive_files,
    )
