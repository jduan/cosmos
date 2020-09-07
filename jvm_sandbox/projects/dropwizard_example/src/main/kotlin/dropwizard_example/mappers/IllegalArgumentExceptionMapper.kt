package dropwizard_example.mappers

import com.codahale.metrics.Meter
import com.codahale.metrics.MetricRegistry
import com.codahale.metrics.MetricRegistry.name
import io.dropwizard.jersey.errors.ErrorMessage
import java.lang.IllegalArgumentException
import javax.ws.rs.core.MediaType
import javax.ws.rs.core.Response
import javax.ws.rs.ext.ExceptionMapper

// You can also override default exception mappers. For example: you can customize responses
// caused by Jackson exceptions:
// public class JsonProcessingExceptionMapper implements ExceptionMapper<JsonProcessingException> {
class IllegalArgumentExceptionMapper(private val metrics: MetricRegistry) : ExceptionMapper<IllegalArgumentException> {
    private val exceptions: Meter = metrics.meter(name(javaClass, "exceptions"))

    override fun toResponse(exception: IllegalArgumentException?): Response {
        exceptions.mark()
        return Response.status(Response.Status.BAD_REQUEST)
            .header("X-YOU-SILLY", "true")
            .type(MediaType.APPLICATION_JSON_TYPE)
            .entity(ErrorMessage(Response.Status.BAD_REQUEST.reasonPhrase))
            .build()
    }
}
