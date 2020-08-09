package jduan

import org.apache.commons.csv.CSVFormat
import org.apache.commons.csv.CSVPrinter
import java.io.FileWriter

data class CsvRecord(
    val project: String,
    val configuration: String,
    val dependency: String,
    val old_version: String,
    val new_version: String,
    val kind: ChangeKind
) {
    fun toList(): List<String> = listOf(project, configuration, dependency, old_version, new_version, kind.toString())
}

enum class ChangeKind {
    Major,
    Minor,
    Patch,
    Added,
    Removed;

    override fun toString(): String =
        when (this) {
            Major -> "Major version change"
            Minor -> "Minor version change"
            Patch -> "Patch version change"
            Added -> "Added depenency"
            Removed -> "Removed depenency"
        }
}
fun main() {
    val outputCsvFile = "/tmp/csv_example.csv"
    val out = FileWriter(outputCsvFile)
    val CSV_FILE_HEADER = listOf(
        "project",
        "configuration",
        "dependency",
        "old_version",
        "new_version",
        "kind"
    )
    val printer = CSVPrinter(out, CSVFormat.DEFAULT.withHeader(*CSV_FILE_HEADER.toTypedArray()))
    val records = listOf(
        CsvRecord("mars", "runtime", "guava", "1.9", "2.3", ChangeKind.Added),
        CsvRecord("mars", "runtime", "guava", "1.9", "2.3", ChangeKind.Removed),
        CsvRecord("mars", "runtime", "guava", "1.9", "2.3", ChangeKind.Major),
        CsvRecord("mars", "runtime", "guava", "1.9", "2.3", ChangeKind.Minor)
    ).sortedBy { record -> record.kind }
    printer.printRecords(records.map { it.toList() })
    printer.flush()
}
