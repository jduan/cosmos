java_binary(
    name = "ProjectRunner",
    srcs = glob(["src/main/java/com/example/ProjectRunner.java"]),
    main_class = "com.example.ProjectRunner",
    deps = [":greeter"],
)

java_library(
    name = "greeter",
    srcs = ["src/main/java/com/example/Greeting.java"],
    # By default targets are only visible to other targets in the same BUILD
    # file.
    visibility = ["//src/main/java/com/example/cmdline:__pkg__"],
)
