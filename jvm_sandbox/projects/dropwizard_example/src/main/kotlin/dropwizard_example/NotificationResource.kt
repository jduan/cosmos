package dropwizard_example

import com.codahale.metrics.annotation.Timed
import io.dropwizard.jersey.params.IntParam
import java.util.concurrent.atomic.AtomicLong
import javax.ws.rs.Consumes
import javax.ws.rs.DefaultValue
import javax.ws.rs.GET
import javax.ws.rs.Path
import javax.ws.rs.PathParam
import javax.ws.rs.Produces
import javax.ws.rs.QueryParam
import javax.ws.rs.WebApplicationException
import javax.ws.rs.core.MediaType
import javax.ws.rs.core.Response

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
}
