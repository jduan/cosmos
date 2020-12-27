def _impl(ctx):
    """"""
    # declare_file declares the rule creates a file with the given name. Files
    # can't be created outside the current pacakge.
    out = ctx.actions.declare_file(ctx.label.name + ".cc")
    ctx.actions.expand_template(
        output = out,
        # You access this attr via "ctx.file.template" because of
        # "allow_single_file" below.
        template = ctx.file.template,
        substitutions = {"{NAME}": ctx.attr.username}
    )
    return [DefaultInfo(files = depset([out]))]

hello_world = rule(
    implementation = _impl,
    attrs = {
        "username": attr.string(default = "unknown person"),
        "template": attr.label(
            allow_single_file = [".cc.tpl"],
            mandatory = True,
        )
    }
)
