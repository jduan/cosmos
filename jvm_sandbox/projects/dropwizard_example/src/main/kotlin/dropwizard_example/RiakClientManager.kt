package dropwizard_example

import io.dropwizard.lifecycle.Managed

// This is a fake manager. It's used to demonstrate how "managed objects" work.
// If this fails to start, the Dropwizard application won't start either.
class RiakClientManager: Managed {
    override fun start() {
        println("Starting a Riak client")
    }

    override fun stop() {
        println("Stopping a Riak client")
    }
}
