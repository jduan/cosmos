package dropwizard_example

import com.fasterxml.jackson.annotation.JsonProperty

data class Notification(
    @JsonProperty val sender: String,
    @JsonProperty val receiver: String,
    @JsonProperty val message: String
)
