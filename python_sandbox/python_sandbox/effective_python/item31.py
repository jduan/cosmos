from weakref import WeakKeyDictionary


class Homework:
    def __init__(self):
        self._grade = 0

    @property
    def grade(self):
        return self._grade

    @grade.setter
    def grade(self, value):
        if not (0 <= value <= 100):
            raise ValueError("Grade must be between 0 and 100")
        self._grade = value


class Grade:
    def __init__(self):
        self._values = WeakKeyDictionary()

    def __get__(self, instance, owner):
        if instance is None:
            return self
        return self._values.get(instance, 0)

    def __set__(self, instance, value):
        if not (0 <= value <= 100):
            raise ValueError("Grade must be between 0 and 100")
        self._values[instance] = value


class Exam:
    # these are class attributes, which are shared across all instances!
    math_grade = Grade()
    writing_grade = Grade()
    science_grade = Grade()


class RevealAccess:
    def __init__(self, initval=None, name="var"):
        self.val = initval
        self.name = name

    def __get__(self, instance, owner):
        print("Retrieving", self.name)
        return self.val

    def __set__(self, instance, value):
        print("Updating", self.name)
        self.val = value


def main():
    first_exam = Exam()
    first_exam.writing_grade = 82
    first_exam.science_grade = 99
    print("First Writing", first_exam.writing_grade)
    print("First Science", first_exam.science_grade)

    second_exam = Exam()
    second_exam.writing_grade = 75
    print("First Writing", first_exam.writing_grade)
    print("Second Writing", second_exam.writing_grade)


if __name__ == "__main__":
    main()
