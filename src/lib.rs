//! # rust-rdkafka
//! Kafka client library for Rust based on [librdkafka].
//!
//! ## The library
//! `rust-rdkafka` provides a safe Rust interface to librdkafka.
//! It currently exports a subset of the funcionalities provided by librdkafka 0.9.2.
//!
//! `rust-rdkafka` provides low level and high level consumers and producers. Low level:
//!
//! * [`BaseConsumer`]: simple wrapper around the librdkafka consumer. It requires to be
//!   periodically `poll()`ed in order to execute callbacks, rebalances and to receive messages.
//! * [`BaseProducer`]: simple wrapper around the librdkafka producer. As in the consumer case,
//!   the user must call `poll()` periodically to execute delivery callbacks.
//!
//! High level:
//!
//!  * [`StreamConsumer`]: it returns a [`stream`] of messages and takes care of polling the consumer
//!  internally.
//!  * [`FutureProducer`]: it returns a [`future`] that will be completed once the message is
//!  delivered to Kafka (or failed).
//!
//! [`BaseConsumer`]: https://docs.rs/rdkafka/0.6.0/rdkafka/consumer/base_consumer/struct.BaseConsumer.html
//! [`BaseProducer`]: https://docs.rs/rdkafka/0.6.0/rdkafka/producer/struct.BaseProducer.html
//! [`StreamConsumer`]: https://docs.rs/rdkafka/0.6.0/rdkafka/consumer/stream_consumer/struct.StreamConsumer.html
//! [`FutureProducer`]: https://docs.rs/rdkafka/0.6.0/rdkafka/producer/struct.FutureProducer.html
//! [librdkafka]: https://github.com/edenhill/librdkafka
//! [futures]: https://github.com/alexcrichton/futures-rs
//! [`future`]: https://docs.rs/futures/0.1.3/futures/trait.Future.html
//! [`stream`]: https://docs.rs/futures/0.1.3/futures/stream/trait.Stream.html
//!
//! *Warning*: the library is under active development and the APIs are likely to change.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rdkafka = "^0.6.0"
//! ```
//!
//! This crate will compile librdkafka from sources and link it statically to your
//! executable. To compile librdkafka you'll need:
//!
//! * the GNU toolchain
//! * GNU `make`
//! * `pthreads`
//! * `zlib`
//! * `libssl-dev`: optional, *not* included by default (feature: `ssl`).
//! * `libsasl2-dev`: optional, *not* included by default (feature: `sasl`).
//!
//! To enable ssl and sasl, use the `features` field in `Cargo.toml`. Example:
//!
//! ```toml
//! [dependencies.rdkafka]
//! version = "^0.6.0"
//! features = ["ssl", "sasl"]
//! ```
//!
//! ## Compiling from sources
//!
//! To compile from sources, you'll have to update the submodule containing librdkafka:
//!
//! ```bash
//! git submodule update --init
//! ```
//!
//! and then compile using `cargo`, selecting the features that you want. Example:
//!
//! ```bash
//! cargo build --features "ssl sasl"
//! ```
//!
//! ## Examples
//!
//! You can find examples in the `examples` folder. To run them:
//!
//! ```bash
//! cargo run --example <example_name> -- <example_args>
//! ```
//!
//! ## Tests
//!
//! The unit tests can run without a Kafka broker present:
//!
//! ```bash
//! cargo test --lib
//! ```
//!
//! To run the full suite:
//!
//! ```bash
//! cargo test
//! ```
//!
//! In this case there is a broker expected to be running on
//! `localhost:9092`. Travis currently only runs the unit tests.
//!

#[macro_use] extern crate log;
extern crate futures;

extern crate rdkafka_sys;

pub use rdkafka_sys::types as types;

pub mod client;
pub mod config;
pub mod consumer;
pub mod error;
pub mod message;
pub mod metadata;
pub mod producer;
pub mod topic_partition_list;
pub mod util;
