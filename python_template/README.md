# How to run tests?
python -m pytest

Running `pytest` from this directory doesn't work because it doesn't add the
current directory to python's `sys.path` (which is used for searching for
packages). Running `python -m pytest` works because python always add the
current directory to python's search path.
