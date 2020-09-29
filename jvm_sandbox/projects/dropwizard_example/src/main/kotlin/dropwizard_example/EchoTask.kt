package dropwizard_example

import com.google.common.collect.ImmutableMultimap
import io.dropwizard.servlets.tasks.PostBodyTask
import java.io.PrintWriter

// You can trigger this task like this:
// curl -X POST  -H "Content-Type: text/json"  -d '{"name": "John", "age": 99}' http://localhost:8081/tasks/echo
class EchoTask(name: String) : PostBodyTask(name) {
    override fun execute(parameters: ImmutableMultimap<String, String>, body: String, output: PrintWriter) {
        output.println("Received body: $body")
        output.flush()
    }
}
