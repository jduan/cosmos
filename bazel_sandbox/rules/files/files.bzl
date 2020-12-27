def _impl(ctx):
    """This rule generates a text file."""
    out = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(
        output = out,
        # You access attributes via "ctx.attr.username"
        content = "Hello {}!\n".format(ctx.attr.username),
    )
    return [DefaultInfo(files = depset([out]))]


write_file = rule(
    implementation = _impl,
    attrs = {
        # "attr.string()" says the type of the username attr is a string!
        "username": attr.string(default = "John", values = ["John", "Jack", "Jake"]),
    }
)
