# This macro uses the native genrule which is a lot shorter
# genrule is great if the rule is calling a short script.
def archive2(name, files, out):
    # The "native" module isn't implicitly imported in libraries, unlike the
    # BUILD or WORKSPACE files. You need to use "native.genrule".
    native.genrule(
        name = name,
        outs = [out],
        srcs = files,
        # The text here is a template.
        cmd = "zip $(OUTS) $(SRCS)",
    )
