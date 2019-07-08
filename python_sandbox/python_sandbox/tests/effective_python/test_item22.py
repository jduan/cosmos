import unittest
from python_sandbox.effective_python.item22 import (
    SimpleGradebook,
    BySubjectGradebook,
    WeightedGradebook,
)


class TestItem22(unittest.TestCase):
    def test1(self):
        book = SimpleGradebook()
        book.add_student('Isaac Newton')
        book.report_grade('Isaac Newton', 90)
        book.report_grade('Isaac Newton', 95)
        book.report_grade('Isaac Newton', 100)
        book.report_grade('Isaac Newton', 70)
        self.assertEqual(88.75, book.average_grade('Isaac Newton'))

    def test2(self):
        book = BySubjectGradebook()
        student = 'Albert Einstein'
        book.add_student(student)
        book.report_grade(student, 'Math', 75)
        book.report_grade(student, 'Math', 65)
        book.report_grade(student, 'Gym', 90)
        book.report_grade(student, 'Gym', 95)
        self.assertEqual(81.25, book.average_grade(student))

    def test3(self):
        book = WeightedGradebook()
        albert = book.student('Albert Einstein')
        math = albert.subject('Math')
        math.report_grade(80, 0.1)
        math.report_grade(90, 0.2)
        print("Albert's average grade", albert.average_grade())
