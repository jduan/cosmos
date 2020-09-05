package dropwizard_example

import com.codahale.metrics.annotation.ExceptionMetered
import com.codahale.metrics.annotation.Metered
import com.google.common.collect.ImmutableMultimap
import io.dropwizard.servlets.tasks.Task
import java.io.PrintWriter

// This demonstrates how to execute a task via the HTTP admin url
class TruncateDatabaseTask(val database: String, name: String): Task(name) {
    @Metered
    @ExceptionMetered
    override fun execute(parameters: ImmutableMultimap<String, String>, output: PrintWriter) {
        output.println("Truncating database $database")
    }
}
