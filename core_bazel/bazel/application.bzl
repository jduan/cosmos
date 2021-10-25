load(":bazel/archive.bzl", "archive")
load(":bazel/compile.bzl", "compile")
load(":bazel/link.bzl", "link")

# Macros are pure functions that invoke rules to create targets.
def application(name, srcs, hdrs = [], extra_files = []):
    # we want to have unique names; otherwise invoking this macro multiple
    # times would lead to the same target name being created multiple times
    compile_target_name = "{}-compile".format(name)
    link_target_name = "{}-link".format(name)

    compile(
        name = compile_target_name,
        srcs = srcs,
        hdrs = hdrs,
    )

    link(
        name = link_target_name,
        objs = [compile_target_name],
        out = "main",
    )

    archive_files = [link_target_name]
    archive_files.extend(extra_files)

    archive(
        name = name,
        files = archive_files,
    )
