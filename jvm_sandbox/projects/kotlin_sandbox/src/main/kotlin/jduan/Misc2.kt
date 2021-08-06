package jduan
import java.util.concurrent.TimeUnit

// When you define a giant constant map, it's easy to have duplicate keys with
// different values accidentally. One of the values will be used. This function
// allows you to define a list of Pairs (key -> value) and it will combine all
// the values for the same key.
fun main() {
    println(Test.ARG_MAX)
}

class Test {
    companion object {
        val ARG_MAX: Int by lazy {
            val process = ProcessBuilder("getconf", "ARG_MAX")
                .redirectOutput(ProcessBuilder.Redirect.PIPE)
                .redirectError(ProcessBuilder.Redirect.PIPE)
                .start()
            if (!process.waitFor(30, TimeUnit.SECONDS)) {
                process.destroy()
                throw RuntimeException("execution timed out: $process")
            }
            if (process.exitValue() != 0) {
                throw JdepsException("execution failed with code ${process.exitValue()}: ${process.errorStream.bufferedReader().readLines()}")
            }

            process.inputStream.bufferedReader().readLine().toInt()
        }
    }
}

val ADDITIONAL_DEPENDENCIES = listOf(
    // TODO: Dont know why this was needed. Investigate.
    "projects/viaduct/common/mutationvalidators/src/main/kotlin/com/airbnb/viaduct/mutators/validators" to listOf(
        "projects/viaduct/common/components"
    ),
    // Kotlin constant not found by jdeps
    "projects/viaduct/data/loggingcenter" to listOf(
        "projects/viaduct/common/loggingcenter/src/main/kotlin/com/airbnb/viaduct/common/loggingcenter",
    ),
    // Kotlin constant not found by jdeps
    "projects/viaduct/presentation/loggingcenter" to listOf(
        "projects/viaduct/common/loggingcenter/src/main/kotlin/com/airbnb/viaduct/common/loggingcenter",
    ),
    // kotlin typealiases get optimized away by the compiler
    "common/viaduct/src/main/kotlin/com/airbnb/viaduct/niobe/scalar" to listOf(
        "common/viaduct/src/main/kotlin/com/airbnb/viaduct/types",
    ),
    // needed due to kotlin compiler optimization
    "common/viaduct/src/main/kotlin/com/airbnb/viaduct" to listOf(
        "common/kotlin/src/main/kotlin/com/airbnb/common/kotlin/scoping",
    ),
    // due to annotations
    "projects/yunclean/client/src/main/java/com/airbnb/yunclean/client" to listOf(
        "projects/yunclean/client/src/main/java/com/airbnb/yunclean/client/module",
    ),
    // classes referenced in javadoc
    "projects/yunclean/client/src/main/java/com/airbnb/yunclean/client/config" to listOf(
        "projects/yunclean/client/src/main/java/com/airbnb/yunclean/client",
    ),
    // same package at 2 different locations
    "projects/spinaltap/spinaltap-omnes/src/main/java/com/airbnb/spinaltap" to listOf(
        "projects/spinaltap/spinaltap-stream/src/main/java/com/airbnb/spinaltap",
    ),
    "projects/spinaltap/spinaltap-client/src/main/java/com/airbnb/spinaltap/dynamodb" to listOf(
        "projects/spinaltap/spinaltap-model/src/main/thrift:dynamodb_attribute",
        // same package at 2 different locations
        "projects/spinaltap/spinaltap-model/src/main/java/com/airbnb/spinaltap/dynamodb",
    ),
    // same package at 2 different locations
    "projects/spinaltap/spinaltap-client/src/main/java/com/airbnb/spinaltap/oyster" to listOf(
        "projects/spinaltap/spinaltap-model/src/main/java/com/airbnb/spinaltap/oyster",
    ),
    // same package at 2 different locations
    "projects/spinaltap/spinaltap-client/src/main/java/com/airbnb/spinaltap/oyster_dynamodb" to listOf(
        "projects/spinaltap/spinaltap-model/src/main/java/com/airbnb/spinaltap/oyster_dynamodb",
    ),
    "projects/spinaltap/spinaltap-model/src/main/java/com/airbnb/spinaltap/dynamodb" to listOf(
        "projects/spinaltap/spinaltap-model/src/main/thrift:dynamodb_attribute",
    ),
    // same package at multiple different locations
    "projects/spinaltap/spinaltap-omnes/src/main/java/com/airbnb/spinaltap" to listOf(
        "projects/spinaltap/spinaltap-client/src/main/java/com/airbnb/spinaltap",
        "projects/spinaltap/spinaltap-stream/src/main/java/com/airbnb/spinaltap",
    ),
    // same package at 2 different locations
    "projects/spinaltap/spinaltap-stream/src/main/java/com/airbnb/spinaltap" to listOf(
        "projects/spinaltap/spinaltap-client/src/main/java/com/airbnb/spinaltap",
    ),
    // classes referenced in javadoc
    "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/annotation" to listOf(
        "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/store",
        "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/store/initializer",
        "common/hammerspace/src/main/java/com/airbnb/common/hammerspace",
    ),
    // classes referenced in javadoc
    "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/store" to listOf(
        "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/store/initializer",
        "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/store/updater",
    ),
    // classes referenced in javadoc
    "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/dataloaders/v2" to listOf(
        "projects/i18n/phrase-translator/src/main/java/com/airbnb/i18n/phrase_translator/dataloaders",
    ),
    "projects/viaduct/config/src/main/kotlin/com/airbnb/viaduct/config"
        to listOf(
        "projects/content-moderator/client/src/main/java/com/airbnb/contentmoderator/client/config",
        // same package in 2 different locations
        "common/viaduct/src/main/kotlin/com/airbnb/viaduct/config",
    ),
    "projects/viaduct/common/trips/src/main/kotlin/com/airbnb/viaduct/common/trips/utils"
        to listOf("common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125"),
    "projects/viaduct/common/utils/e2elogging/src/main/kotlin/com/airbnb/viaduct/utils"
        to listOf("common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125"),
    "projects/search/framework/common/src/main/java/com/airbnb/search/framework/common" to
        listOf("projects/search/framework/common/src/main/thrift:dojo_field_config"),
    "projects/viaduct/services/viaduct/src/main/kotlin/com/airbnb/viaduct/graphql/instrumentation" to listOf(
        "common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125",
        "projects/viaduct/schema",
        "tools/kotlin",
    ),
    "projects/viaduct/services/viaduct/src/main/kotlin/com/airbnb/viaduct/config" to
        listOf("common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125"),
    "projects/search-platform/common/analytics/src/main/java/com/airbnb/searchplatform/analytics"
        to listOf("common/thrift/thriftsources:hfileservice"),
    "projects/search/common/serviceapi/src/main/java/com/airbnb/search/common/serviceapi/util"
        to listOf("projects/search/samurai/api:thrift"),
    "projects/trips/common/src/main/java/com/airbnb/trips/clients" to listOf(
        "common/thrift/thriftsources:hfileservice",
        "projects/oyster/api/src/main/thrift/common:kv_value",
        "projects/oyster/api/src/main/thrift:kvstore_service",
    ),
    "projects/goblin/lib/src/main/java/com/airbnb/goblin/lib/converter"
        to listOf("projects/search/samurai/api:thrift"),
    "projects/discover/hfileservice/src/main/java/com/airbnb/discover/hfileservice/client/airbnb"
        to listOf("common/thrift/thriftsources:hfileservice"),
    "projects/discover/hfileservice/src/main/java/com/airbnb/discover/hfileservice/client/dao"
        to listOf("common/thrift/thriftsources:hfileservice"),
    "projects/discover/hfileservice/src/main/java/com/airbnb/discover/hfileservice/tools" to listOf(
        "common/thrift/thriftsources:discovery",
        "common/thrift/thriftsources:hfileservice",
    ),
    "projects/discover/hfileservice/src/main/java/com/airbnb/discover/hfileservice" to listOf(
        "common/thrift/thriftsources:hfileservice",
        "projects/oyster/api/src/main/thrift/common:kv_value",
        "projects/oyster/api/src/main/thrift:kvstore_service",
    ),
    "projects/satori/coordinator/api/src/main/java/com/airbnb/satori/coordinator/response" to listOf(
        "projects/satori/data_thrift:autocommplete_ranking_rule",
        "projects/satori/data_thrift:infix_index_documents",
        "projects/satori/data_thrift:location_documents",
        "projects/satori/data_thrift:pdp_documents",
        "projects/satori/data_thrift:playlist_documents",
        "projects/satori/data_thrift:refinement_location",
        "projects/satori/data_thrift:satori_parameters",
        "projects/satori/data_thrift:site_navigation_documents",
        "projects/satori/data_thrift:user_input_features",
    ),
    "projects/viaduct/common/components/src/main/kotlin/com/airbnb/viaduct/components/omni/models"
        to listOf("common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125"),
    "projects/viaduct/common/components/src/main/kotlin/com/airbnb/viaduct/components/guestplatform"
        to listOf("common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125"),
    "projects/micasa/service-common/src/main/java/com/airbnb/micasa/transcoders/idl" to listOf(
        "projects/shareddao/micasadb/schema/src/main/java/com/airbnb/shareddao/micasadb/models",
        "projects/shareddao/micasadb/schema/src/main/java/com/airbnb/shareddao/micasadb/tables",
        "projects/shareddao/micasadb/schema/src/main/thrift:canonical_description",
        "projects/shareddao/micasadb/schema/src/main/thrift:hierarchy_edge",
        "projects/shareddao/micasadb/schema/src/main/thrift:hierarchy_tree",
        "projects/shareddao/micasadb/schema/src/main/thrift:hosting_amenity",
        "projects/shareddao/micasadb/schema/src/main/thrift:hosting_wireless_info",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_category",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_description",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_document",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_expectation",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_info",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_last_action",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_misc",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_relationship",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_ugc_text",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_user_group",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_version",
        "projects/shareddao/micasadb/schema/src/main/thrift:merchandising_content",
        "projects/shareddao/micasadb/schema/src/main/thrift:room",
        "projects/shareddao/micasadb/schema/src/main/thrift:room_amenity",
        "projects/shareddao/micasadb/schema/src/main/thrift:room_description",
    ),
    "projects/shareddao/micasadb/schema/src/main/java/com/airbnb/shareddao/micasadb/models" to listOf(
        "projects/shareddao/micasadb/schema/src/main/thrift:canonical_description",
        "projects/shareddao/micasadb/schema/src/main/thrift:hierarchy_edge",
        "projects/shareddao/micasadb/schema/src/main/thrift:hierarchy_tree",
        "projects/shareddao/micasadb/schema/src/main/thrift:hosting_amenity",
        "projects/shareddao/micasadb/schema/src/main/thrift:hosting_wireless_info",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_category",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_description",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_document",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_expectation",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_info",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_last_action",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_misc",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_relationship",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_ugc_text",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_user_group",
        "projects/shareddao/micasadb/schema/src/main/thrift:listing_version",
        "projects/shareddao/micasadb/schema/src/main/thrift:merchandising_content",
        "projects/shareddao/micasadb/schema/src/main/thrift:room",
        "projects/shareddao/micasadb/schema/src/main/thrift:room_amenity",
        "projects/shareddao/micasadb/schema/src/main/thrift:room_description",
    ),
    "projects/shareddao/common/src/main/java/com/airbnb/shareddao/common/utils"
        to listOf("projects/shareddao/common/util/src/main/java/com/airbnb/shareddao/common/utils"),
    "projects/shareddao/pricingdb/dtos/src/main/java/com/airbnb/shareddao/pricingdb/dtos"
        to listOf("projects/shareddao/pricingdb/datasource/src/main/java/com/airbnb/shareddao/pricingdb/schema"),
    "projects/search/samurai/api/src/main/java/com/airbnb/search/samurai/api"
        to listOf("projects/search/samurai/api/src/main/thrift"),
    "projects/search/framework/query/src/main/java/com/airbnb/search/framework/query" to listOf(
        "projects/search/framework/common/src/main/thrift:dojo_field_config",
        "projects/search/framework/query/src/main/thrift",
    ),
    "projects/nebula/utils/src/main/java/com/airbnb/nebula/utils"
        to listOf("projects/nebula/common/src/main/java/com/airbnb/nebula/utils"),
    "common/locations/markets/src/main/java/com/airbnb/locations/markets"
        to listOf("schemas/locations_markets:locations_markets_data"),
    "projects/security/cipher-client/src/main/java/com/airbnb/cipher/api"
        to listOf("projects/security/cipher-common/src/main/java/com/airbnb/cipher/api"),
    "common/services-platform/common/src/main/java/com/airbnb/services/framework/idldatafactory"
        to listOf("common/services-platform/common/src/main/java/com/airbnb/services/framework/jackson"),
    "common/graphql-legacy-scalars/src/main/java/com/airbnb/graphqllegacy/apiv3/schema"
        to listOf("common/service-framework-util/src/main/java/com/airbnb/services/data"),
    "common/dropwizard1.0/src/main/java/com/airbnb/common/dropwizard"
        to listOf(
        "common/dropwizard1.0/src/main/java/com/airbnb/common/dropwizard/thrift",
        "common/service-framework-util/src/main/java/com/airbnb/services/data",
    ),
    "common/dropwizard1.0/src/main/java/com/airbnb/common/dropwizard/modules"
        to listOf("common/metrics-util/src/main/java/com/airbnb/common/metrics"),
    "common/graphql-legacy/src/main/java/com/airbnb/graphqllegacy/apiv3/schema"
        to listOf("common/graphql-legacy-scalars/src/main/java/com/airbnb/graphqllegacy/apiv3/schema"),
    "common/service-framework-util/src/main/java/com/airbnb/services/data"
        to listOf("tools/swift/swift-codec"),
    "common/services-platform/services/src/main/java/com/airbnb/services/framework/automated_testing"
        to listOf("common/services-platform/common/src/main/java/com/airbnb/services/framework/jackson"),
    "common/viaduct"
        to listOf("common/kotlin/src/main/kotlin/com/airbnb/common/kotlin/scoping"),
    "projects/apple/src/main/java/com/airbnb/apple/powergrid"
        to listOf("projects/apple/src/main/java/com/airbnb/apple/api"),
    "projects/wormhole/common"
        to listOf("common/service-framework-util/src/main/java/com/airbnb/common/services"),
    "projects/trebuchet/simple-strategy/src/main/java/com/airbnb/trebuchet/simplestrategy/strategy/mobile"
        to listOf("projects/trebuchet/simple-strategy/src/main/java/com/airbnb/trebuchet/simplestrategy/utils"),
    "projects/i18n/client/src/main/java/com/airbnb/i18n/config"
        to listOf("projects/i18n/common/src/main/java/com/airbnb/i18n/config"),
    "projects/i18n/client/src/main/java/com/airbnb/i18n"
        to listOf("projects/i18n/common/src/main/java/com/airbnb/i18n"),
    "projects/viaduct/services/viaduct/src/main/kotlin/com/airbnb/viaduct/cache"
        to listOf("common/viaduct/src/main/kotlin/com/airbnb/viaduct/cache"),

    "common/service-framework/src/main/java/com/airbnb/common/services"
        to listOf("schemas/request_context:request_context_data"),
    "projects/apple/src/main/java/com/airbnb/apple/api"
        to listOf("schemas/apple:apple_data"),
    "projects/i18n/common/src/main/java/com/airbnb/i18n"
        to listOf("schemas/i18n:i18n_data"),
    "common/thrift/utils/src/main/java/com/airbnb/common/thrift/utils"
        to listOf("common/thrift/thriftsources:discovery"),

    // Legacy Thrift dependencies
    "common/hadoop/src/main/java/com/airbnb/common/hadoop/hdfs/utils"
        to listOf("common/thrift/thriftsources:hfileservice"),
    "projects/oyster/impl/oyster-kvstore/mussel-rpc/common/src/main/java/com/airbnb/mussel/rpc/common"
        to listOf("projects/oyster/api/src/main/thrift/common:exceptions"),
    "projects/oyster/impl/oyster-kvstore/mussel-rpc/thrift/dispatch/src/main/java/com/airbnb/mussel/rpc/thrift/dispatch/lib"
        to listOf(
        "common/thrift/thriftsources:hfileservice",
        "projects/hfiles/commons/src/main/java/com/airbnb/hfiles/commons",
        "projects/oyster/api/src/main/thrift:kvstore_service"
    ),
    "projects/oyster/impl/oyster-kvstore/oyster-kvstore-client/src/main/java/com/airbnb/mussel/derived/client"
        to listOf(
        "projects/oyster/api/src/main/thrift:derived_structure",
        "projects/oyster/api/src/main/thrift/common:kv_value",
        "projects/oyster/api/src/main/thrift:kvstore_service",
        // NOTE: this is imported from Gradle
        "projects/oyster/impl/oyster-kvstore/derived-data-schema:id_list_result"
    ),
    "projects/oyster/impl/oyster-kvstore/oyster-kvstore-client/src/main/java/com/airbnb/oyster/kvstore/client"
        to listOf(
        "projects/oyster/api/src/main/thrift:derived_structure",
        "projects/oyster/api/src/main/thrift:kvstore_service",
        "projects/oyster/api/src/main/thrift/common:exceptions",
        "projects/oyster/api/src/main/thrift/common:kv_value"
    ),
    "projects/oyster/impl/oyster-kvstore/oyster-kvstore-client/src/main/java/com/airbnb/oyster/kvstore/client/lib"
        to listOf("projects/oyster/api/src/main/thrift/common:exceptions"),
    "projects/oyster/model/src/main/java/com/airbnb/oyster/model"
        to listOf(
        "projects/oyster/api/src/main/thrift/dispatcher:filterexpression",
        "projects/oyster/api/src/main/thrift/common:value",
        "projects/oyster/api/src/main/thrift/dispatcher:dispatcher_service_modify",
        "projects/oyster/api/src/main/thrift/dispatcher:dispatcher_service_query",
    ),
    "projects/search/common/serviceapi/src/main/java/com/airbnb/search/common/serviceapi"
        to listOf(
        "projects/search/common/constants/src/main/java/com/airbnb/search/common/constants",
        "projects/search/samurai/api:thrift"
    ),

    // Same java package split across Gradle projects
    "projects/oyster/impl/common/cluster-state/helix/src/main/java/com/airbnb/oyster/common/cluster/state"
        to listOf(
        "projects/oyster/impl/common/cluster/src/main/java/com/airbnb/oyster/common/cluster/state",
        "projects/oyster/impl/common/cluster-state/common/src/main/java/com/airbnb/oyster/common/cluster/state"
    ),

    "projects/plus_common/src/main/java/com/airbnb/plus/common/daos"
        to listOf("projects/plus_common:thrift"),
    "projects/plus_common/src/main/java/com/airbnb/plus/common/pluscities"
        to listOf(
        "projects/plus_common:thrift",
        "projects/oyster/model/src/main/java/com/airbnb/oyster/model"
    ),
    "projects/plus_common/src/main/java/com/airbnb/plus/common/pluscities/dao"
        to listOf("projects/plus_common:thrift"),
    "projects/search/samurai/api/src/main/java/com/airbnb/search/samurai/api"
        to listOf("projects/search/samurai/api:thrift"),

    // Kotlin constant not found by jdeps
    "projects/plus_common/src/main/java/com/airbnb/plus/common/clients"
        to listOf("projects/plus_common/src/main/java/com/airbnb/plus/common/constants"),

    // Scala package objects
    "projects/tns/libs/data/src/main/scala/com/airbnb/kyoo/data"
        to listOf("projects/tns/libs/data/src/main/scala/com/airbnb/kyoo/decisions"),
    "projects/tns/libs/data/src/main/scala/com/airbnb/trust/data"
        to listOf(
        "projects/tns/libs/ghostingappeals/src/main/java/com/airbnb/trust/ghostingappeals/datautil",
        "schemas/compliance_common:metrics_data",
        "schemas/trust_common:client_action_data",
    ),

    // Annotations not found by jdeps
    "projects/trips/bankai/api/filters/src/main/java/com/airbnb/bankai/api/filters/models"
        to listOf(
        "projects/trips/bankai/api/filters/src/main/java/com/airbnb/bankai/api/filters/models/checkboxs",
        "projects/trips/bankai/api/filters/src/main/java/com/airbnb/bankai/api/filters/models/switches",
    ),

    // Undetected viaduct dependencies
    "projects/viaduct/services/viaduct/src/main/kotlin/com/airbnb/viaduct"
        to listOf("projects/viaduct/services/viaduct/src/main/kotlin/com/airbnb/viaduct/resources"),
    "projects/viaduct/presentation/listyourspace" to listOf(
        "projects/listing-common/lib/src/main/java/com/airbnb/listing/lib/utils/property",
        "common/kotlin/src/main/kotlin/com/airbnb/common/kotlin/metrics",
    ),
    "projects/viaduct/data/experiences"
        to listOf("projects/experiences/common/src/main/java/com/airbnb/experiences/common/constants"),
    "projects/viaduct/presentation/resourcecenter"
        to listOf("projects/viaduct/common/components"),
    "projects/viaduct/presentation/pdp/stays" to listOf(
        "projects/listing-common/lib/src/main/java/com/airbnb/listing/lib/utils",
        "projects/viaduct/common/currency/src/main/kotlin/com/airbnb/viaduct/loaders",
        "projects/viaduct/common/utils/e2elogging/src/main/kotlin/com/airbnb/viaduct/utils",
    ),
    "projects/viaduct/data/reviews" to listOf(
        "projects/listing-common/lib/src/main/java/com/airbnb/listing/lib/utils",
        "projects/viaduct/common/currency/src/main/kotlin/com/airbnb/viaduct/loaders",
        "projects/viaduct/common/scrubber/src/main/kotlin/com/airbnb/viaduct/loaders",
    ),
    "projects/viaduct/presentation/hostmessaging"
        to listOf("projects/hostinbox/common/src/main/java/com/airbnb/hostinbox/api/constants"),
    "projects/viaduct/presentation/myp"
        to listOf("projects/listing-common/constants/src/main/java/com/airbnb/listing/constants"),
    "projects/viaduct/presentation/payouts" to listOf(
        "projects/payments/instruments-common/src/main/java/com/airbnb/instruments/common/util",
        "common/locations/location-util/src/main/java/com/airbnb/locations/util",
    ),

    // Viaduct test fixtures
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/cache"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/instrumentation"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/instrumentation/riskcheck"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/instrumentation/riskcheck/prefilter"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/idlwarmup"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/resources/ratelimiter"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/sitar"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/utilapp"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/utilapp/datadog"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "projects/viaduct/services/viaduct/src/test/kotlin/com/airbnb/viaduct/utils"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/cache"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/derived" to listOf(
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
        "common/schema/src/main/resources/com/airbnb/common/schema/graphql/schema:viaduct_custom_schema_objects_lib_125",
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct:viaduct_lib",
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct/fields/generated/base",
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct/schema/base:base_lib",
    ),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/instrumentation"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/instrumentation"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/mutators"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/policy"
        to listOf(
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/sitar",
    ),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/policy/china"
        to listOf(
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct/policy:policy_lib",
    ),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/policy/media"
        to listOf(
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
    ),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/schema/wiring"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/types"
        to listOf(
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct/schema/generated/typeresolvers",
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/schema/generated",
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
    ),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/types/fields"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/types/resolver"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/niobe/scalar"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/niobe"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/execution"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/graphql/execution/middleware"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/sitar"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/operation"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/loaders/core/nodes"
        to listOf("common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing"),
    "common/viaduct/src/test/kotlin/com/airbnb/viaduct/components/base"
        to listOf(
        "common/viaduct/src/test/kotlin/com/airbnb/viaduct/components/factory:factory_lib",
        "common/viaduct/src/testFixtures/kotlin/com/airbnb/viaduct/testing",
    ),
)

