# This macro uses the native genrule which is a lot shorter
def archive2(name, files, out):
    native.genrule(
        name = name,
        outs = [out],
        srcs = files,
        cmd = "zip $(OUTS) $(SRCS)",
    )
