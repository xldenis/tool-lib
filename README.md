# Experimental tool-lib

This is an experimental implementation of the `tool_lib` discussed as part of [Project Stable MIR](https://github.com/rust-lang/project-stable-mir).
The objective is to identify what subset of the `rustc` API is used by verification tools and *why*.

The initial plan is to first *identify* and *isolate* the set of rust apis used by verification tools, and then classify those apis into "Necessary", "Questionable" and "Problematic" categories so that we can identify what should really be exposed via a `tool_lib` and what should either be wrapped or reformulated.

Additionally, this could form a good starting place for code sharing, by allowing tool authors to place common utility functions in a shared location.