# https://nikgrozev.com/2013/12/10/monads-in-15-minutes/
# What's a Monad?
# In essence, the "bind" function implements the glue code and the "unit" function implements
# the initialization code. This allows us to solve the problem in one line:
# bind(bind(... bind(unit(x), f1), f2) ... fn)
#
# See the "pipeline" function below.
#
# In order to compose the "bind" and "unit" functions, the return types of unit and bind, and
# the type of bind's first parameter must be compatible. This is called the "Monadic Type".
# In other words, the types of all the interim values must be Monadic.
#
# The monadic type for these 3 examples are:
#   * a pair of (int, string)
#   * a list of [x, [x])
#   * a value that is an employee, or wage, or None (this is allowed by dynamic languages)


def main():
    add_all(10)
    print("===================\n\n")
    add_all2(10)
    print("===================\n\n")

    list_interim_values(100)
    print("===================\n\n")
    list_interim_values2(100)
    print("===================\n\n")
    list_interim_values3(100)
    print("===================\n\n")

    assert get_boss_wage(None) is None
    print("===================\n\n")
    assert get_boss_wage2(None) is None
    print("===================\n\n")
    employee = Employee("John", 10)
    boss = Employee("Jack", 100)
    employee.boss = boss
    assert get_boss_wage(employee) == 100
    assert get_boss_wage2(employee) == 100
    print("===================\n\n")
    assert get_boss_wage2(boss) is None
    assert get_boss_wage2(boss) is None
    print("===================\n\n")


def add1(x):
    return x + 1, str(x) + " + 1"


def add2(x):
    return x + 2, str(x) + " + 2"


def add3(x):
    return x + 3, str(x) + " + 3"


def add_all(x):
    log = "Ops:\n"
    res, log1 = add1(x)
    log += log1 + ";\n"

    res, log2 = add2(res)
    log += log2 + ";\n"

    res, log3 = add3(res)
    log += log3 + ";\n"

    print(log)
    print("final result: %s" % res)


# Example 1
def unit(x):
    return x, "Ops:\n"


def bind(t, f):
    res = f(t[0])
    return res[0], t[1] + res[1] + ";\n"


def add_all2(x):
    """
    Same as "add_all" but in a monadic way. This is better because the "bind" function implements
    all the glue code and we don't have to repeat it. If we have add4, we can just include it
    in the sequence here easily.
    """
    res, log = bind(bind(bind(unit(x), add1), add2), add3)
    print(log)
    print("final result: %s" % res)


def inc1(x):
    return x + 1


def inc2(x):
    return x + 2


def inc3(x):
    return x + 3


def list_interim_values(x):
    """
    Compute x + 1 + 2 + 3. Additionally, generate a list of all interim and final values:
    x, x + 1, x + 1 + 2, x + 1 + 2 + 3
    """
    interim = [x]
    res = inc1(x)
    interim.append(res)
    res = inc2(res)
    interim.append(res)
    res = inc3(res)
    interim.append(res)
    print("final result: %s" % res)
    print("interim values: %s" % interim)


# Example 2
def unit2(x):
    return [x, [x]]


def bind2(res, f):
    interim = res[1]
    next_value = f(res[0])
    interim.append(next_value)
    return [next_value, interim]


def list_interim_values2(x):
    """ Monadic way """
    pair = bind2(bind2(bind2(unit2(x), inc1), inc2), inc3)
    print("final result: %s" % pair[0])
    print("interim values: %s" % pair[1])


def list_interim_values3(x):
    """ Monadic way """
    pair = pipeline(x, unit2, bind2, bind2, bind2)
    print("final result: %s" % pair[0])
    print("interim values: %s" % pair[1])


def pipeline(value, *funcs):
    """ Given an initial value and a list of functions, apply the functions repeatedly and
    return the final result."""
    ret = value
    for func in funcs:
        ret = func(ret)
    return ret


class Employee:
    def __init__(self, name, wage, boss=None):
        self.name = name
        self.wage = wage
        self.boss = boss

    def get_boss(self):
        return self.boss

    def get_wage(self):
        return self.wage


def get_boss_wage(employee):
    """
    Return the wage of the given employee's boss.
    If the employee is None, or the employee's boss is None, return None.
    """
    wage = None
    if employee is not None:
        boss = employee.get_boss()
        if boss is not None:
            wage = boss.get_wage()

    return wage


# Example 3
def unit3(employee):
    return employee


def bind3(employee, f):
    if employee is None:
        return None
    else:
        return f(employee)


def get_boss_wage2(employee):
    """ Monadic version. """
    return bind3(bind3(unit3(employee), Employee.get_boss), Employee.get_wage)


if __name__ == '__main__':
    main()
