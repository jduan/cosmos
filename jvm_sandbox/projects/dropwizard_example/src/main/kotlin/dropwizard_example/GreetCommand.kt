package dropwizard_example

import io.dropwizard.cli.Command
import io.dropwizard.setup.Bootstrap
import net.sourceforge.argparse4j.inf.Namespace
import net.sourceforge.argparse4j.inf.Subparser

// This shows how to add a Command.
class GreetCommand(name: String = "greet", description: String = "Prints a greeting") : Command(name, description) {
    override fun run(bootstrap: Bootstrap<*>, namespace: Namespace) {
        println("Hello ${namespace.getString("user")}")
    }

    override fun configure(subparser: Subparser) {
        subparser.addArgument("-u", "--user")
                .dest("user")
                .type(String::class.java)
                .required(true)
                .help("The user of the program")
    }
}
