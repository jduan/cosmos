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

# Use 7zip to archive files
def _seven_zip(ctx):
    out = ctx.actions.declare_file("{}.7z".format(ctx.label.name))
    inputs = [ctx.file.version_data]
    inputs.extend(ctx.files.archive_files)

    args = ctx.actions.args()
    args.add(out)
    args.add(ctx.file.version_data)
    args.add_all(ctx.files.archive_files)

    ctx.actions.run(
        executable = ctx.executable._seven_zip_binary,
        arguments = [args],
        inputs = inputs,
        outputs = [out],
    )

    return [DefaultInfo(files = depset([out]))]

seven_zip = rule(
    implementation = _seven_zip,
    attrs = {
        "_seven_zip_binary": attr.label(
            # this label points to an executable
            executable = True,
            default = ":_seven_zip_binary",
            # executables must specify the "cfg" attribute: it can be either
            # host or target
            cfg = "host",
        ),
        "version_data": attr.label(
            allow_single_file = True,
            mandatory = True,
        ),
        "archive_files": attr.label_list(
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
    },
)
