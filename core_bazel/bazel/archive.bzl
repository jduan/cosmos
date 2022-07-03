def _archive(ctx):
    print("name: %s" % ctx.label.name)

    # declare_file says this rule will create an output file with the given name
    out = ctx.actions.declare_file("%s.zip" % ctx.label.name)

    # args() returns an Arg object that can be used to build memory-efficient command lines
    args = ctx.actions.args()
    args.add(out)

    # why ctx.files.files? "ctx.files.param_name" is used to access the attr
    # in this case, the "param_name" is called "files".
    # btw, "ctx.attr.files" is the label itself, not the actual resolved files!
    args.add_all(ctx.files.files)

    # this adds an action to the action graph
    ctx.actions.run(
        executable = "zip",
        arguments = [args],
        # The "input dependencies of the action node" in the graph. If these are
        # missing, the files will not be copied to the sandbox folder.
        inputs = ctx.files.files,
        outputs = [out],
    )

    # You can return multiple things that are usable in downstream rules.
    # Each item in this list is called a "provider".
    return [DefaultInfo(files = depset([out]))]

# This registers a rule definition and defines the args of the rule.
archive = rule(
    implementation = _archive,
    attrs = {
        # attr is used to define the type of the argument
        "files": attr.label_list(allow_files = True),
    },
)
