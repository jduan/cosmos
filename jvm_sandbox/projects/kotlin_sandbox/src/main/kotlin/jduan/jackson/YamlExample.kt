package jduan.jackson

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import java.io.File
import java.math.BigDecimal

fun main() {
    val file = File("projects/kotlin_sandbox/src/main/kotlin/jduan/jackson/orders.yaml")
    val order = Order.parseStringToYaml(file)
    println(order)

    val file2 = File("projects/kotlin_sandbox/src/main/kotlin/jduan/jackson/application-spec.yaml")
    val appSpec = AppSpec.load(file2)
    println(appSpec)

    val file3 = File("projects/kotlin_sandbox/src/main/kotlin/jduan/jackson/key_reference_map.yaml")
    val yamlObjectMapper = ObjectMapper(YAMLFactory()).registerKotlinModule()
    val map = yamlObjectMapper.readValue(file3, Map::class.java)
}

data class AppSpec(
    val name: String,
    val service_dependency: List<String>,
    val service_idl: List<String>,
    // optional field
    val oyster_model: List<String>?
) {
    companion object {
        fun load(file: File): AppSpec {
            val yamlObjectMapper = ObjectMapper(YAMLFactory()).registerKotlinModule()
            return yamlObjectMapper.readValue<AppSpec>(file, AppSpec::class.java)
        }
    }
}

data class OrderLine(
    val item: String,
    val quantity: Int,
    val unitPrice: BigDecimal
)
data class Order(
    val orderNo: String,
    val customerName: String,
    val orderLines: List<OrderLine>
) {
    companion object {
        fun parseStringToYaml(file: File): Order {
            val yamlObjectMapper = ObjectMapper(YAMLFactory()).registerKotlinModule()
            return yamlObjectMapper.readValue<Order>(file, Order::class.java)
        }
    }
}