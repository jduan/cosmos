## How does this thing work?

This project is set up by following https://github.com/apache/thrift/tree/master/tutorial/rs.

## The essential steps are:
* Install the "thrift" compiler using homebrew
* Generate the source code from .thrift files:

    thrift --out src/ --gen rs -r tutorial.thrift
    
* Copy the server.rs and client.rs files from https://github.com/apache/thrift/tree/master/tutorial/rs/src/bin

## How to start the server?

    cargo run --bin server
    
## How to run the client?

    cargo run --bin client
    
