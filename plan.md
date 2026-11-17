# Desugaring and Macros

## Desugaring

Many languages are structured as a *core language* and *syntactic sugar*. A first step of compilation "desugars", translating programs that use syntactic sugar to use only the core language.

This is especially true of Scheme and Racket! Almost everything is defined as syntactic sugar over a very small core.

## Macros

A *macro expander* makes the desugaring pass of a compiler extensible.

Racket lets us define such extensions---macros---right inside our programs.

We might write a language extension just for use in a single program!

Two interesting language design questions:
* What data representation should macro expansion use?
* What language should macro transformations be written in?

Plan:

desugaring
c-macros
for/list
or
rust/

## Macros in industrial languages

Rust and Scala are modern industrial languages where macros are used heavily, adding custom syntax or domain-specific languages for:

- Unit testing
- Serialization
- Foreign language bindings
- Database queries
- Embedded HTML and JavaScript
