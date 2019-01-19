package jduan.objects

data class Person(val name: String, val hourlyRate: Double)

object Payroll {
    val allEmployees = arrayListOf<Person>()

    fun addEmployee(employee: Person) {
        allEmployees.add(employee)
    }

    fun calculateSalary(): Double {
        var totalSalary = 0.0
        for (person in allEmployees) {
            totalSalary += (person.hourlyRate * 40)
        }

        return totalSalary
    }
}
