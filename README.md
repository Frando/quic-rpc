# Quic-Rpc

A streaming rpc system based on quic

[![Latest Version]][crates.io] [![Docs Badge]][docs.rs] ![Crates.io](https://img.shields.io/crates/l/quic-rpc)

[Latest Version]: https://img.shields.io/crates/v/quic-rpc.svg
[crates.io]: https://crates.io/crates/quic-rpc
[Docs Badge]: https://img.shields.io/badge/docs-docs.rs-green
[docs.rs]: https://docs.rs/quic-rpc


## Goals

### Interaction patterns

Provide not just request/response RPC, but also streaming in both directions, similar to [grpc].

- 1 req -> 1 res
- 1 req, update stream -> 1 res
- 1 req -> res stream
- 1 req, update stream -> res stream

It is still a RPC system in the sense that interactions get initiated by the client.

### Transports

- memory transport with very low overhead. In particular, no ser/deser, currently using [flume]
- quic transport via the [quinn] crate
- transparent combination of the above

### API

- The API should be similar to the quinn api. Basically "quinn with types".

## Non-Goals

- Cross language interop. This is for talking from rust to rust
- Any kind of verisoning. You have to do this yourself
- Making remote message passing look like local async function calls
- Being runtime agnostic. This is for tokio

## Example

[computation service](https://github.com/n0-computer/quic-rpc/blob/main/tests/math.rs)

[quinn]: https://docs.rs/quinn/
[flume]: https://docs.rs/flume/
[grpc]: https://grpc.io/