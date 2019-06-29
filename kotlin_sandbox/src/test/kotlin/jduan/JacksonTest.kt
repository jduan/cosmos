package jduan

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import junit.framework.TestCase.assertEquals
import org.junit.Test
import java.net.URLDecoder

class JacksonTest {
    @Test
    fun `hello world`() {
        val yamlObjectMapper = ObjectMapper(YAMLFactory()).registerKotlinModule()
        val str = yamlObjectMapper.writeValueAsString(setOf("hello world", "airbnb rocks"))
        val test = "com.airbnb.verity.specs.PermissionControlEdgesDAOSpec#the+dao+returns+a+list+of+edges+by+given+user+id"
        val decodedName = URLDecoder.decode(test, "UTF-8")
        println("decodedName: $decodedName")
        assertEquals("Hello, world!", "Hello, world!")
    }
}
