from python_sandbox import app
# from . import app
# from .app import run
# from python_sandbox.app import run
import os
import time


def allocate_large_memory():
    buffer = []
    # every element occupies roughly 40MB
    # so this allocates roughly 10gb of ram
    items = 1000 * 1000 * 1000
    print("Start allocating memory")
    for i in range(items):
        buffer.append(i)
    print("Done allocating memory")
    hour = 3600
    duration = 10 * hour
    time.sleep(duration)


# __name__ is a special string attribute of every Python module.
# Its value would be "python_sandbox.utils.say_hello" in
# "python_sandbox/utils/say_hello.py".
# When a module is run directly via python -m some_module, that module is
# assigned a special value of __name__ == '__main__'
if __name__ == '__main__':
    # app.run()
    # print("Hello, world!")
    print("Process ID: %s" % os.getpid())
    allocate_large_memory()
