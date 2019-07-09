# Things to Remember
#
# Private attributes aren't rigorously enforced by the Python compiler.
#
# Plan from the beginning to allow subclasses to do more with your internal APIs and attributes
# instead of locking them out by default. Use "protected attributes" in this case.
#
# Use documentation of protected fields to guide subclasses instead of trying to force access
# control with private attributes.
#
# Only consider using private attributes to avoid naming conflicts with subclasses that are out of
# your control.


# The private attribute behavior is implemented with a simple transformation of the attribute name.
# When the Python compiler sees private attribute access in methods like
# MyChildObject.get_private_field, it translates __private_field to access
# _MyChildObject__private_field instead. In this example, __private_field was only defined in
# MyParentObject.__init__, meaning the private attribute’s real name is
# _MyParentObject__private_field. Accessing the parent’s private attribute from the child class
# fails simply because the transformed attribute name doesn’t match.
#
# Knowing this scheme, you can easily access the private attributes of any class, from a subclass or
# externally, without asking for permission.
#
# Why doesn’t the syntax for private attributes actually enforce strict visibility? The simplest
# answer is one often-quoted motto of Python: “We are all consenting adults here.” Python
# programmers believe that the benefits of being open outweigh the downsides of being closed.
#
# protected attributes: _value
# private attributes: __value
from pprint import pprint


class MyObject:
    def __init__(self):
        self.public_field = 5
        self.__private_field = 10

    def get_private_field(self):
        return self.__private_field

    # Class methods have access to private attributes because they are declared
    # within the surrounding class block.
    @classmethod
    def get_private_field_of_instance(cls, instance):
        return instance.__private_field


class MyChildObject(MyObject):
    # subclass can't access its parent class' private fields
    # this would throw:
    # AttributeError: 'MyChildObject' object has no attribute '_MyChildObject__private_field'
    def get_private_field(self):
        return self.__private_field


def main():
    foo = MyObject()
    assert foo.public_field == 5
    assert foo.get_private_field() == 10
    # the following throw an exception:
    # AttributeError: 'MyObject' object has no attribute '__private_field'
    # foo.__private_field

    assert MyObject.get_private_field_of_instance(foo) == 10

    child = MyChildObject()
    # the following throw an exception:
    # AttributeError: 'MyChildObject' object has no attribute '_MyChildObject__private_field'
    # child.get_private_field()

    # but this works!
    assert child._MyObject__private_field == 10

    # this reveals private attributes:
    # {
    #   '_MyObject__private_field': 10,
    #   'public_field': 5
    # }
    pprint(child.__dict__)


if __name__ == '__main__':
    main()
