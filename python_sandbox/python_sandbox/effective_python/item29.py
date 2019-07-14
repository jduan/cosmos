# Things to Remember
#
# Define new class interfaces using simple public attributes, and avoid set and get methods.
#
# Use @property to define special behavior when attributes are accessed on your objects, if
# necessary.
#
# Follow the rule of least surprise and avoid weird side effects in your @property methods.
# Getters should not update states.
#
# Ensure that @property methods are fast; do slow or complex work using normal methods.

# Use plain attributes instead of getters and setters!

# In Python, however, you almost never need to implement explicit setter or getter methods.
# Instead, you should always start your implementations with simple public attributes. Later,
# if you decide you need special behavior when an attribute is set, you can migrate to the
# @property decorator and its corresponding setter attribute.


class Resistor:
    def __init__(self, ohms):
        self.ohms = ohms
        self.voltage = 0
        self.current = 0

    def __repr__(self):
        return "Resistor(ohms: %r, voltage: %r, current: %r)" % \
               (self.ohms, self.voltage, self.current)


class VoltageResistance(Resistor):
    def __init__(self, ohms):
        super().__init__(ohms)
        self._voltage = 0

    @property
    def voltage(self):
        return self._voltage

    @voltage.setter
    def voltage(self, voltage):
        self._voltage = voltage
        self.current = self._voltage / self.ohms


class BoundedResistance(Resistor):
    def __init__(self, ohms):
        super().__init__(ohms)

    @property
    def ohms(self):
        return self._ohms

    @ohms.setter
    def ohms(self, ohms):
        """
        You can validate input in setters.
        """
        if ohms <= 0:
            raise ValueError('%f ohms must be > 0' % ohms)
        self._ohms = ohms


class FixedResistance(Resistor):
    def __init__(self, ohms):
        super().__init__(ohms)

    @property
    def ohms(self):
        return self._ohms

    @ohms.setter
    def ohms(self, ohms):
        """
        You can validate input in setters.
        """
        if hasattr(self, '_ohms'):
            raise AttributeError("Can't set attribute because it's set already.")
        self._ohms = ohms


def main():
    r1 = Resistor(50e3)
    r1.ohms = 10e3
    print(r1)
    r1.ohms += 5e3
    print(r1)

    r2 = VoltageResistance(1e3)
    print("Before: %5r amps" % r2.current)
    r2.voltage = 10
    print("After: %5r amps" % r2.current)
    print("r2: %r" % r2)

    r3 = FixedResistance(1e3)
    print("r3: %r" % r3)
    # the next line would cause an exception
    # r3.ohms = 2e3


if __name__ == '__main__':
    main()
