load("//:bazel/counter.bzl", "Counter")

def _count_items(ctx):
    item_count = len(ctx.attr.items)
    return [Counter(count = item_count)]

count_items = rule(
    implementation = _count_items,
    attrs = {
        "items": attr.int_list(),
    },
)
