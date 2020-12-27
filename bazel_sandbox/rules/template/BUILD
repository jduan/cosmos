load("//template:files.bzl", "hello_world")

hello_world(
    name = "hello",
    username = "Alice",
    template = "file.cc.tpl",
)

cc_binary(
    name = "hello_bin",
    srcs = [":hello"],
)
