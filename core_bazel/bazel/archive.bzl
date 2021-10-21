def _archive(ctx):
    print("name: %s" % ctx.label.name)
    out = ctx.actions.declare_file("%s.zip" % ctx.label.name)
    args = ctx.actions.args()
    args.add(out)
    args.add_all(ctx.files.files)

    ctx.actions.run(
        executable = "zip",
        arguments = [args],
        inputs = ctx.files.files,
        outputs = [out],
    )

    return [DefaultInfo(files = depset([out]))]

# This registers a rule definition and defines the args of the rule.
archive = rule(
    implementation = _archive,
    attrs = {
        "files": attr.label_list(allow_files = True),
    },
)
