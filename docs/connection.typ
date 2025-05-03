#import "@preview/touying:0.6.1": pause

== Connection Configuration

=== Transport Protocols
- *TCP*: Reliable, stream-based transport
- *QUIC*: Modern transport with built-in encryption & multiplexing
- *WebSocket*: Browser-compatible transport

=== Multiplexing
- *Yamux*: "Yet Another Multiplexer":Multiple logical streams over single connection, flow control & backpressure handling.
- *mplex*: Simple multiplexer protocol, lighter than Yamux, but less feature-rich.

=== Security Layer

*Noise Protocol*: Allows for secure key exchange and authentication between peers, utilizing their keypairs also used for identification.

