package dropwizard_example

import com.fasterxml.jackson.module.kotlin.KotlinModule
import dropwizard_example.mappers.IllegalArgumentExceptionMapper
import io.dropwizard.Application
import io.dropwizard.setup.Bootstrap
import io.dropwizard.setup.Environment

class HelloWorldApplication : Application<HelloWorldConfiguration>() {
    override fun initialize(bootstrap: Bootstrap<HelloWorldConfiguration>) {
        super.initialize(bootstrap)
        bootstrap.addCommand(GreetCommand())
        bootstrap.objectMapper.registerModule(KotlinModule())
    }

    override fun getName(): String {
        return "Hello, world!"
    }

    override fun run(configuration: HelloWorldConfiguration, environment: Environment) {
        val healthCheck = TemplateHealthCheck(configuration.getTemplate())
        environment.jersey().register(
            HelloWorldResource(configuration.getTemplate(), configuration.getDefaultName())
        )
        val store = NotificationStore()
        environment.jersey().register(
            NotificationResource(configuration.getTemplate(), configuration.getDefaultName(), store)
        )
        environment.jersey().register(
            IllegalArgumentExceptionMapper(environment.metrics())
        )
        environment.healthChecks().register("template", healthCheck)
        environment.lifecycle().manage(RiakClientManager())
        environment.admin().addTask(TruncateDatabaseTask("order", "truncate"))
        environment.admin().addTask(EchoTask("echo"))
    }
}
