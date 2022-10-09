# How to run tests?
python -m pytest

# Python Data Model

The first thing to know about special methods (__len__ etc) is that they are meant
to be called the python interpreter, and not by you. You don't write `obj.__len__()`.
You write `len(obj)`.

More often than not, the special method call is implicit. For example, the statement
`for i in x:` actually causes the invocation of `iter(x)`, which in turn may call
`x.__iter__()` if that's available, or use `x.__getitem__()` as a fallback.

You should be implementing them more often than invoking them explicitly.

## __repr__ vs __str__
https://stackoverflow.com/questions/1436703/what-is-the-difference-between-str-and-repr

* The default implementation is useless (it’s hard to think of one which wouldn’t be, but yeah)
* __repr__ goal is to be unambiguous
* __str__ goal is to be readable

## Collections API

The "Collection" ABC (abstract base class) has 3 parents:
* Iterable (`__iter__`): to support the `for` construct
* Sized (`__len__`): to support the `len` function
* Container (`__contains__`): to support the `in` operator

The "Collection" ABC (abstract base class) has 3 children:
* Sequence (__getitem__, __iter__, etc)
* Mapping (__getitem__, __contains__, etc)
* Set

There's also a `Reversible` ABC and `Sequence` is a child of it.

Note that python doesn't require concrete classes to actually inherit from any of
these ABCs. Any class that implements `__len__` satisfies the `Sized` interface.
