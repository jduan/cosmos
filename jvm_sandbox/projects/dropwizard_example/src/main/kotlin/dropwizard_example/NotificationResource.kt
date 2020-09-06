package dropwizard_example

import com.codahale.metrics.annotation.Timed
import io.dropwizard.jersey.params.IntParam
import org.slf4j.LoggerFactory
import java.util.concurrent.atomic.AtomicLong
import javax.validation.Valid
import javax.validation.constraints.NotNull
import javax.ws.rs.Consumes
import javax.ws.rs.DefaultValue
import javax.ws.rs.GET
import javax.ws.rs.POST
import javax.ws.rs.Path
import javax.ws.rs.PathParam
import javax.ws.rs.Produces
import javax.ws.rs.QueryParam
import javax.ws.rs.WebApplicationException
import javax.ws.rs.core.MediaType
import javax.ws.rs.core.Response
import javax.ws.rs.core.UriBuilder

@Path("/{user}/notifications")
@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.APPLICATION_JSON)
class NotificationResource(private val template: String, private val defaultName: String) {
    private val counter: AtomicLong = AtomicLong()

    @GET
    @Timed
    fun sayHello(
        @PathParam("user") user: String?,
        // "IntParam" means this query param must be an integer
        // There are all kinds of "XXXParam" types. See io.dropwizard.jersey.params.AbstractParam.
        // You can encapsulate the vast majority of your validation logic using these type params.
        @QueryParam("count") @DefaultValue("1") count: IntParam
    ): Saying {
        val value = (1..count.get()).map {
            String.format(template, user ?: defaultName)
        }.joinToString(separator = " ")
        if (user == "baduser") {
            throw WebApplicationException(Response.Status.NOT_FOUND)
        }
        return Saying(counter.incrementAndGet(), value)
    }

    @POST
    fun add(
        @PathParam("user") user: String?,
        // Jersey maps the request entity to any single, unbound parameter. Here, it will
        // deserialize the requqest into a Notification object.
        @NotNull @Valid notification: Notification
    ): Response {
        logger.info("Adding a notification: $notification")
        return Response.created(
            UriBuilder.fromResource(NotificationResource::class.java)
                .build(user))
            .build()
    }

    companion object {
        private val logger = LoggerFactory.getLogger(NotificationResource::class.java)
    }
}
