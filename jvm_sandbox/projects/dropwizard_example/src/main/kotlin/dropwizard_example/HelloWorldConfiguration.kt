package dropwizard_example

import com.fasterxml.jackson.annotation.JsonProperty
import io.dropwizard.Configuration

class HelloWorldConfiguration : Configuration() {
    private lateinit var template: String
    private lateinit var defaultName: String

    @JsonProperty
    fun getTemplate() = template

    @JsonProperty
    fun setTemplate(template: String) {
        this.template = template
    }

    @JsonProperty
    fun getDefaultName() = defaultName

    @JsonProperty
    fun setDefaultName(name: String) {
        this.defaultName = name
    }
}
