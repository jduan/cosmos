# from python_sandbox.utils.say_hello import hello
from python_sandbox.arborist.analyze_broken_builds2 import (
    get_builds_from_redis,
    analyze_particular_task_root_cause,
    analyze_failure_reasons,
    analyze_compilation_failures,
    analyze_other_failures,
)

NUM_OF_DAYS = 1


def run():
    builds = get_builds_from_redis(NUM_OF_DAYS)
    print("Got %s broken builds for the past %s days" % (len(builds), NUM_OF_DAYS))
    analyze_particular_task_root_cause(builds)
    print("======== failure reasons ===========")
    analyze_failure_reasons(builds)
    print()
    print("======== compilation failures ===========")
    analyze_compilation_failures(builds)
    print()
    print("======== other failures ===========")
    analyze_other_failures(builds)
    print()
