package com.graphqljava.tutoria.bookdetails

import com.google.common.io.Resources
import com.google.common.io.Resources.getResource
import graphql.GraphQL
import graphql.schema.GraphQLSchema
import graphql.schema.idl.RuntimeWiring
import graphql.schema.idl.SchemaGenerator
import graphql.schema.idl.SchemaParser
import graphql.schema.idl.TypeDefinitionRegistry
import graphql.schema.idl.TypeRuntimeWiring.newTypeWiring
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.context.annotation.Bean
import org.springframework.stereotype.Component


@Component
class GraphQLProvider(@Autowired val graphQLDataFetchers: GraphQLDataFetchers) {
    private lateinit var graphQL: GraphQL

    init {
        val url = getResource("schema.graphqls")
        val sdl = Resources.toString(url, Charsets.UTF_8)
        val graphQLSchema = buildSchema(sdl)
        graphQL = GraphQL.newGraphQL(graphQLSchema).build()
    }

    @Bean
    fun graphQL(): GraphQL {
        return graphQL
    }

    private fun buildSchema(sdl: String): GraphQLSchema {
        val typeRegistry: TypeDefinitionRegistry = SchemaParser().parse(sdl)
        val runtimeWiring: RuntimeWiring = buildWiring()
        val schemaGenerator = SchemaGenerator()
        return schemaGenerator.makeExecutableSchema(typeRegistry, runtimeWiring)
    }

    private fun buildWiring(): RuntimeWiring {
        return RuntimeWiring.newRuntimeWiring()
            .type(newTypeWiring("Query")
                .dataFetcher("bookById", graphQLDataFetchers.bookByIdDataFetcher))
            .type(newTypeWiring("Book")
                .dataFetcher("author", graphQLDataFetchers.authorDataFetcher))
            .build()
    }
}
