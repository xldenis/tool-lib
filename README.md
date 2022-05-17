# Experimental tool-lib

This is an experimental implementation of the `tool_lib` discussed as part of [Project Stable MIR](https://github.com/rust-lang/project-stable-mir).
The objective is to identify what subset of the `rustc` API is used by verification tools and *why*.

The initial plan is to first *identify* and *isolate* the set of rust apis used by verification tools, and then classify those apis into "Necessary", "Questionable" and "Problematic" categories so that we can identify what should really be exposed via a `tool_lib` and what should either be wrapped or reformulated.

Additionally, this could form a good starting place for code sharing, by allowing tool authors to place common utility functions in a shared location.

As the work progresses, it could also be a good opportunity re-organize the various APIs into a coherent and unified set of modules, masking their underlying `rustc` crate sources.

## A Rough Guideline for adding an API

- Avoid exposing entire _modules_.
- Avoid using glob patterns to expose types or functions. 
- Avoid exposing constructors of types, instead expose the type itself.
