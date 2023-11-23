# Runtime
To be able to handle transportation, call system calls and message parsing,
a runtime that handles it for you is necessary
(and also convenient, so you don't write your own without being a need).


## Core Runtime
So we have a core runtime(written in Rust) that will provide you the facilities
as a base of any feature.

This runtime can and is used as a basis for other language specific runtimes of comline,
so for example lets say you need to use C++, the comline C++ runtime will speak to the
core runtime, and the core will speak back so everything works together for the language
you are using.


## Any Language Runtimes
Runtimes can be different depending on what kind of language you are using, compiled AOT,
compiled JIT, interpreted runtime, etc...

[//]: # (Decide if this bit of text is necessary)
As a user of comline you won't need to worry because it will make the integration seamless
and without issues.
