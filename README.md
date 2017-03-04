# Tokio-based EventStore client API written in Rust

[EventStore](https://geteventstore.com/) is an open-source immutable event database.
EventStore allows writing to named event streams with optional optimistic locking and reading from a stream or a stream of all events in the database (not yet implemented in this project).
This project aims to be a driver with similar features to the .NET client API but at the moment it has the simplest operations implemented:

 * read an event
 * read a stream forward or backward
 * write events

Supported server version: currently this has been developed against the latest stable 3.9.3.

The suffix `-tcp` refers to the fact that EventStore also has AtomPub-based HTTP API.
There exists already a crate for that: [http_event_store](https://crates.io/crates/http_event_store).
The TCP protocol is multiplexed, custom framed with the payloads specified as protobuf messages.
Some of the payloads are exposed by the current version of this API.

## Examples

Please see the documentation (hopefully soon at docs.rs).

The repository also includes an aspiring command line client under `examples/testclient`.

# Unimplemented features in order of importance:

 1. read events from `$all` stream
 2. deleting a stream
 3. volatile subscriptions:
   * refactoring to use `tokio_proto::streaming::multiplex` instead of `tokio_proto::multiplex`
   * current messages are ok as headers, but appeared events could probably be body chunks
   * maintaining a subscription by pumping events to a `futures::sink::Sink`, detecting overflows and dropping the subscription
 4. Less of directly using the protobuf messages in the API
 5. Cleaning up the message builders
 6. Hide the use of `Package` from users
 7. Add some "operation" API so that user does not need to `match package.message {}`
 8. Nice API which would not require users to run the `tokio_core::reactor::Core``

## "Perhaps later" features:

 1. persistent subscriptions (haven't researched this yet)
 2. competing consumers (?)
 3. long running transactions (???)

# Building

`cargo build` will handle building, and testclient becomes usable after building it in it's own directory: `cd examples/testclient && cargo run -- --help`.

# Testing

Currently `cargo test` runs very primitive codec tests to ensure everything pretty much works.
Also, I'm not sure if there is any point in aiming at 100% coverage en/decoding protobuf messages.
Perhaps later on if server version differences are discovered these will make more sense.

The `examples/testclient` contains `test_with_inmemory_es.bash` which will do some smoke testing against an EventStore instance expected to be running at `127.0.0.1:1113`.
The script also includes code to fetch but not run an inmemory instance as it's currently impossible to download EventStore binaries from an https:// host (if you know one, please let me know) and either way this should probably be done with some jailing/sandboxing to be accessible.

Simple no-brainer way to run the server is to spin-up a local ubuntu VM based on something similar to a `VagrantFile` below (assuming you have [vagrant](https://vagrantup.com) and [VirtualBox](https://www.virtualbox.org/wiki/VirtualBox) ready to go):

```rb
Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"
  config.vm.network "forwarded_port", guest: 1113, host: 1113
  config.vm.provider "virtualbox" do |vb|
    vb.gui = false
    vb.memory = "2048"
  end
end
```

Then download the latest EventStore tar.gz and execute it with `./run-node.sh --ext-ip 0.0.0.0 --mem-db`.

# Rebuilding the `client_messages`

 1. Obtain [ClientMessageDTOs.proto](https://github.com/EventStore/EventStore/blob/master/src/Protos/ClientAPI/ClientMessageDtos.proto) or [raw link](https://raw.githubusercontent.com/EventStore/EventStore/master/src/Protos/ClientAPI/ClientMessageDtos.proto)
 2. Checkout latest `quick-protobuf`: `git clone https://github.com/tafia/quick-protobuf`
 3. `cd quick-protobuf/codegen`
 4. `cargo run --single-mod --output $es_tcp_checkout/src/client_messages.rs ClientMessageDTOs.proto`

