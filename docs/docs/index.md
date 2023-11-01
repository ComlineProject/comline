# Welcome to Comline
Comline is an agnostic schema, RPC/IPC (or any other similar terminologies)
library, similar to ones like:

 - [Protobuf](https://protobuf.dev/), [Capn'Proto](https://capnproto.org/), [Fuchsia IDL (FIDL)](https://fuchsia.dev/fuchsia-src/get-started/sdk/learn/fidl)
 - [Apache Avro](https://avro.apache.org/), [Apache Thrift](https://thrift.apache.org/docs/idl)


## What constitutes Comline?

### [Interface Definition Language (IDL or also named Schema)](./idl/index.md)
Some of the distinct differences to other libraries is that this
[IDL/Schema](./idl/index.md) allows to be more flexible, optional or robust, 
detailed and specific when setting options, rules, message structures,
protocols (named services in protobuf, interfaces in capn'proto) and so on...


### [Intermediate Representation (IR)](./ir/index.md)
Schemas compile into an Intermiate Representation that solidifies the structure as a sort of "mini-spec",
which then can output to other formats like a rpc schema such as
[OpenRPC](https://open-rpc.org/), a custom text format, or better a binary
compact format like [msgpack](https://msgpack.org/) which is ideal to reduce wire load


### [Call System (CaSy)](./casy/index.md)
Yet another one is that the call system can be provided and the library is able
to do the intended routing, so specs like [json-rpc](https://www.jsonrpc.org/)
can be used, or a custom format that is binary compact like msgpack, or any 
custom call format that you may need to use or implement

Even another one is that the message serialization can be specified so that
you can use any data format, from JSON to even other schema library's formats,
or once again binary minimalist formats like msgpack 
(might sound repetitive, but it's just a good format for many cases)


### [Code Generation (Codegen)](./codegen/index.md)
Codegen is a common feature that similar libraries have, however
some of them lack details at development time and might make the experience
more indicated towards "try-it-out" and not provide so much detail,

Comline tries to provide more language specific generation that fits as best,
detailed and information specific to them as possible, be it for compiled
or dynamic languages, it's believed to be important to have as much information
detail as possible at development type.
