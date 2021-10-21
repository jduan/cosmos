def _archive(ctx):
    out_file = ctx.actions.declare_file(ctx.attr.out)
    args = ctx.actions.args()
    args.add(out_file)
    args.add_all(ctx.files.files)

    ctx.actions.run(
        executable = "zip",
        arguments = [args],
        inputs = ctx.files.files,
        outputs = [out_file],
    )

    return [DefaultInfo(files = depset([out_file]))]

# This registers a rule definition and defines the args of the rule.
archive = rule(
    implementation = _archive,
    attrs = {
        "files": attr.label_list(allow_files = True),
        "out": attr.string(mandatory = True),
    },
)
