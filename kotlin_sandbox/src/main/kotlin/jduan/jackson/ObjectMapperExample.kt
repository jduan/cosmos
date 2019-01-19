package jduan.jackson

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.fasterxml.jackson.module.kotlin.registerKotlinModule

// Car has an Engine field.
data class Car(val color: String, val type: String, val engine: Engine)

data class Engine(val power: Int)

fun main(args: Array<String>) {
    objectToJSON()
    jsonToObject()
    jsonToObject2()
}

private fun objectToJSON() {
    val objectMapper = ObjectMapper()
    val car = Car("yellow", "renault", Engine(300))
    // objectMapper.writeValue(System.out, car)
    // or
    println("json string: ${objectMapper.writeValueAsString(car)}")
}

private fun jsonToObject() {
    val objectMapper = jacksonObjectMapper()
    val jsonString = """
        {
          "color": "Black",
          "type": "BMW",
          "engine": {
            "power": 300
          }
        }
    """.trimIndent()
    val car = objectMapper.readValue<Car>(jsonString)
    println(car)
}

@JsonIgnoreProperties(ignoreUnknown = true)
data class BrokenBuildSummary(
    val isSuccessful: Boolean,
    val id: String,
    val failureReason: String,
    val failedTasks: String,
    val taskRootCause: String,
    val failedTests: String,
    val testFailureRootCause: String
)

// handles deserialization of Date object
private fun jsonToObject2() {
    val jsonString = """
            {"id":"p4iyglex7ggbo","startTime":"1546631314739","duration":"94092","tasks":":projects:demogorgon:test, :projects:discover:atlas:test, :projects:discover:common:api:test, :projects:discover:common:atlas_client:test, :projects:discover:common:avro:test, :projects:discover:common:lib:test, :projects:discover:common:location:test, :projects:discover:common:test, :projects:discover:hermes:test, :projects:discover:hfileservice:hfileservice-airlab-utils:test, :projects:discover:hfileservice:test, :projects:discover:test, :projects:dora:api:test, :projects:dora:service:test, :projects:dora:test, :projects:dora:utils:test, :projects:dove:streaming:test, :projects:dove:test, :projects:dr-backup-master:test, :projects:drishti:test, :projects:dumbledore:test, :projects:ebert:api:test, :projects:ebert:client:test, :projects:ebert:service:test, :projects:ebert:test, :projects:eggplant:dao:test, :projects:eggplant:test, :projects:erf:erf-client:test, :projects:erf:test, :projects:eve:test, :projects:event-o-matic:test, :projects:falcon:test, :projects:fapiao:test, :projects:farmersmarket:test, :projects:fengshui-event:test, :projects:fengshui:test, :projects:fiducia:test, :projects:forest:test, :projects:fourier:server:test, :projects:fourier:test, :projects:gaia-client:test, :projects:gaia-common:test, :projects:gaia-core:test, :projects:gaia:test, :projects:gandalf:api:test, :projects:gandalf:client:test, :projects:gandalf:test, :projects:geo-api:test, :projects:geo-client:test, :projects:geo-common:test, :projects:geo:test, :projects:goblin:api:test, :projects:goblin:lib:test, :projects:goblin:sitar-provider:test, :projects:goblin:test, :projects:golden-gate:test, :projects:goldfinger:test, :projects:gomera:test, :projects:gringott:acceptance-test:test, :projects:gringott:api:test, :projects:gringott:client:test, :projects:gringott:common:test, :projects:gringott:mutation-publisher:test, :projects:gringott:service:test, :projects:gringott:test, :projects:griphook-pricing-data-service-export:test, :projects:griphook:api:test, :projects:griphook:client:test, :projects:griphook:griphook-mutation-publisher:test, :projects:griphook:internal-common:test, :projects:griphook:pricing-representation:test, :projects:griphook:service:test, :projects:griphook:test, :projects:groupie:test, :projects:growth:core:test, :projects:growth:datasets:adbasis:test, :projects:growth:datasets:foundation:test, :projects:growth:datasets:hello-world:test, :projects:growth:datasets:hive-exports:test, :projects:growth:datasets:listing-feed:test, :projects:growth:datasets:roxbury:test, :projects:growth:datasets:runner:test, :projects:growth:datasets:sem-bidding:test, :projects:growth:datasets:stat-mta:test, :projects:growth:datasets:test, :projects:yuanbao:test","customValues":"(Git SHA, 93331457f4e122874c9fa5888626b1613fd2cf95), (Git branch, refs/heads/master), (Hostname, bf2eac6f1376), (shadow.:projects:oyster:impl:oyster-kvstore:derived-data-schema:shadowJar.dependencies, 63), (shadow.:projects:oyster:impl:oyster-kvstore:derived-data-schema:shadowJar.relocations, com/airbnb → shadow/com/airbnb\ncom/amazonaws → shadow/com/amazonaws\ncom/fasterxml → shadow/com/fasterxml\ncom/google → shadow/com/google\ncom/twitter → shadow/com/twitter\norg/jboss → shadow/org/jboss\norg/slf4j → shadow/org/slf4j), (shadow.:projects:oyster:impl:oyster-kvstore:derived-data-schema:shadowJar.configurations, runtimeClasspath)","isSuccessful":"false","isCI":"true","failureReason":"TEST_FAILURE","failedTasks":":projects:event-o-matic:test","taskRootCause":"There were failing tests. See the report at: file:///jorb/repo/projects/event-o-matic/build/reports/tests/test/index.html","failedTests":"com.airbnb.eventomatic.api.EventOMaticAPIServiceIDLTest#testCreatePayoutTrace_HappyCase","testFailureRootCause":"expected:<PayoutTraceResponse{eventTimestamp=2019-01-04T19:49:28Z, payoutTrace=PayoutTraceLog{payoutToken=PayoutToken{value=767812, id=1, name=billToken, type=String}, payoutTrace=PayoutTrace{method=TraceMethod{value=ENVOY, id=1, name=method, type=WellKnownTraceMethod}, state=REQUESTED, initiatedBy=Admin{userId=7216959}}}}> but was:<PayoutTraceResponse{eventTimestamp=2019-01-04T19:49:27Z, payoutTrace=PayoutTraceLog{payoutToken=PayoutToken{value=767812, id=1, name=billToken, type=String}, payoutTrace=PayoutTrace{method=TraceMethod{value=ENVOY, id=1, name=method, type=WellKnownTraceMethod}, state=REQUESTED, initiatedBy=Admin{userId=7216959}}}}>"}
    """.trimIndent()
    val objectMapper = ObjectMapper().registerKotlinModule()
    val brokenBuildSummary = objectMapper.readValue<BrokenBuildSummary>(jsonString)
    println(brokenBuildSummary)
}
