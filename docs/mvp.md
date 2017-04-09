Minimal Viable Product
================================================================================

Goal
--------------------------------------------------------------------------------

To produce a specification and working program with the minimal amount of functionality required to assemble a large sample program (Sonic 1 for SEGA Master System), with the aim to expand the specification and program further after this goal is met.

What's Required
--------------------------------------------------------------------------------

* Above all, the processing of **Z80 assembly instructions** and parameters, which can include machine registers "`HL`", memory dereferences "`[...]`", and numeric & symbol references

* **Constants** require a minimal amount of implementation as they do not change value and are globally scoped, not requiring a module system to be in place

* A means of **specifying the linking order**, or deferring to a built-in algorithm. In order to build an original Sonic 1 ROM we'll need to specify the ROM location of every label.

    This isn't practical for building new software so a linking algorithm can be provided later but isn't absolutely required for the MVP

* Handling of **banked ROM sections**. It is not clear yet how banked sections will be speified in the language grammar. There are different banking schemes that need to be covered including single, double and contiguous banks

What Is Not Required
--------------------------------------------------------------------------------

* User-defined **Functions** can be avoided to simplfy implementation and a set of built-in funtions can provide the most required functionality with a short-cut native implementation rather than a real fetch-execute cycle

* **Macros** -- especially [hygienic macros](1) -- are very complex as they intersect assembly, function and macro domains so require a large specification surface area

* A **compiler, VM and external linker** are not required as we can execute the AST directly to produce the in-memory binaries and link them there without the need of an intermediate linker format

[1]: https://en.wikipedia.org/wiki/Hygienic_macro


