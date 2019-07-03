from python_sandbox import app
# from . import app
# from .app import run
# from python_sandbox.app import run


# __name__ is a special string attribute of every Python module.
# Its value would be "python_sandbox.utils.say_hello" in
# "python_sandbox/utils/say_hello.py".
# When a module is run directly via python -m some_module, that module is
# assigned a special value of __name__ == '__main__'
if __name__ == '__main__':
    app.run()
