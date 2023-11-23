# Codegen (Code Generation)
When you define a schema in comline and build it, the result will be
an Intermediate Representation of the schema code
([read the explanation about IR](../ir/index.md) in case you need to 
know what it is).

Once having an IR we can generate equivalent code for any language,
so aspects like typing, implementation details and code for a language
can be generated, even for different versions of comline schemas,
and language versions.


## Motivations

[//]: # (TODO: This was a snippet of a conversation, make it proper for docs)
Comparing to other libraries:
as for example in contrast compared with Protobuf's gRPC codegen
implementations for some languages that might lack more concrete details,
they mildly specify Message typing, but do not provide enough Service
typing information to help IDE tools in showing or optionally enforcing
through abstraction interfaces (if the language has them)

