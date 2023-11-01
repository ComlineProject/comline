# Intermediate Representation Generation

When compiling a schema, the schemas imported within it and details
like metadata, state and so on are written into a lockfile `state.lock`

This lockfile will contain frozen information about everything to be worried,
the goal being to preserve relations, state, versioning of the schema(and related
schemas) past and present

Upon locking, any changes made to any of the schemas and their relations, the state
and the differential changes are tracked, and at the same time automatic version and
state changes will be recorded


## Version State
Versioning follows SemVer (Semantic Versioning)

When adding new fields to a any of the [Components](#Components), the minor version
will be increased by 1 in the minor segment

When making any change to an existing field in any of the [Components](#Components), the
major version will be increaded by 1 in the major segment


**IMPORTANT TODO**: This makes `schema.ids` not need indexing manually anywhere,
so the convention `#1 optional ...` where `#1` is indexing, can be removed entirely
(if the system above is defined to substitute this feature, thread carefully)


**NOTE TODO**: The idea of a lockfile is to create state freezing, it can be
a specific IR unit format of itself, or maybe a Json one? decide this


### Components
A component is any part of the schema that does declarations, such being:
 - namespace
 - constant
 - settings
 - structure
 - enum
 - error
 - validator
 - protocol

