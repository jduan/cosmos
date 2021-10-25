def _archive(ctx):
    print("name: %s" % ctx.label.name)

    # declare_file says this rule will create an output file with the given name
    out = ctx.actions.declare_file("%s.zip" % ctx.label.name)

    # args() returns an Arg object that can be used to build memory-efficient command lines
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
        # attr is used to define the type of the argument
        "files": attr.label_list(allow_files = True),
    },
)
