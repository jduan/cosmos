# Things to Remember
#
# Avoid using multiple inheritance if mix-in classes can achieve the same outcome.
#
# Use pluggable behaviors at the instance level to provide per-class customization when mix-in
# classes may require it.
#
# Compose mix-ins to create complex functionality from simple behaviors.

# Python is an object-oriented language with built-in facilities for making multiple inheritance
# tractable (see Item 25: “Initialize Parent Classes with super”). However, it’s better to avoid
# multiple inheritance altogether.

# If you find yourself desiring the convenience and encapsulation that comes with multiple
# inheritance, consider writing a mix-in instead. A mix-in is a small class that only defines a set
# of additional methods that a class should provide. Mix-in classes don’t define their own instance
# attributes nor require their __init__ constructor to be called.

# Writing mix-ins is easy because Python makes it trivial to inspect the current state of any object
# regardless of its type. Dynamic inspection lets you write generic functionality a single time, in
# a mix-in, that can be applied to many other classes. Mix-ins can be composed and layered to
# minimize repetitive code and maximize reuse.
from pprint import pprint
import json


class ToDictMixin(object):
    def to_dict(self):
        return self._traverse_dict(self.__dict__)

    def _traverse_dict(self, instance_dict):
        output = {}
        for key, value in instance_dict.items():
            output[key] = self._traverse(key, value)
        return output

    def _traverse(self, key, value):
        if isinstance(value, ToDictMixin):
            return value.to_dict()
        elif isinstance(value, dict):
            return self._traverse_dict(value)
        elif isinstance(value, list):
            return [self._traverse(key, i) for i in value]
        elif hasattr(value, '__dict__'):
            return self._traverse_dict(value.__dict__)
        else:
            return value


# Note how the JsonMixin class defines both instance methods and class methods. Mix-ins let you add
# either kind of behavior.
class JsonMixin:
    @classmethod
    def from_json(cls, data):
        kwargs = json.loads(data)
        return cls(**kwargs)

    def to_json(self):
        return json.dumps(self.to_dict())


class BinaryTree(ToDictMixin):
    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right


class BinaryTreeWithParent(BinaryTree):
    def __init__(self, value, left=None, right=None, parent=None):
        super().__init__(value, left=left, right=right)
        self.parent = parent

    def _traverse(self, key, value):
        if isinstance(value, BinaryTreeWithParent) and key == 'parent':
            return value.value
        else:
            return super()._traverse(key, value)


# Mix-ins can be composed together.
class DatacenterPack(ToDictMixin, JsonMixin):
    def __init__(self, switch=None, machines=None):
        self.switch = Switch(**switch)
        self.machines = [Machine(**kwargs) for kwargs in machines]


class Switch(ToDictMixin, JsonMixin):
    def __init__(self, ports, speed):
        self.ports = ports
        self.speed = speed


class Machine(ToDictMixin, JsonMixin):
    def __init__(self, cores, ram, disk):
        self.cores = cores
        self.ram = ram
        self.disk = disk


def main():
    tree = BinaryTree(10,
                      left=BinaryTree(7, right=BinaryTree(9)),
                      right=BinaryTree(13, left=BinaryTree(11)))
    pprint(tree.to_dict())

    root = BinaryTreeWithParent(10)
    root.left = BinaryTreeWithParent(7, parent=root)
    root.left.right = BinaryTreeWithParent(9, parent=root.left)
    pprint(root.to_dict())

    serialized = """{
        "switch": {"ports": 5, "speed": 1e9},
        "machines": [
            {"cores": 8, "ram": 32e9, "disk": 5e12},
            {"cores": 4, "ram": 16e9, "disk": 1e12},
            {"cores": 2, "ram": 4e9, "disk": 500e9}
        ]
    }"""
    datacenter_pack = DatacenterPack.from_json(serialized)
    roundtrip = datacenter_pack.to_json()
    assert json.loads(serialized) == json.loads(roundtrip)


if __name__ == '__main__':
    main()
