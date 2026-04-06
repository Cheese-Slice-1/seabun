<sup><sub><sup><sub> note: almost everything is exclussively in lowercase or uppercase because of the vibes i want the language to have: unserious, informal and friendly. also i won't take responsibility for the disgust, anger, frustration, or anything else you may feel reading this text :Þ </sub></sup></sub></sup>

# seabun!

<sup><sub>(todo: logo image)</sub></sup>

## introduction

seabun (lowercase everything!) is a somewhat silly programming language i made for fun :3

i designed it to be as dumb but also somewhat readable as i could make it

## design philosophy

seabun aims to be _different_ (not better, not cool, just different), breaking some common practices for the sole purpose of diverging from "the norm"

some of these "strange" design choices include:

- having main be a keyword followed by a single expression (similar to a label)
  - `main ...`
  - `main { ... }`
- using a period (.) instead of a semicolon (;)
- making lambdas the default function type
- using `: ...`, `: ... !` and `: ... ?` instead of `( ... )` when calling and defining functions
  - `let f = fun: x T, ... do ...`
  - `let f = fun: x T, ... { ... }`
  - `let f = fun: x T, ... -> T do ...`
  - `let f = fun: x T, ... -> T { ... }`
  - `f: x, ... !`
  - `f: x, ... ?`
  - `f: x, ...`
    - `!` and `?` can only be dropped when the call is by itself (not "nested"). that means `first: second!.` is the same as `first: second!!.`
    - for function declarations, `!` and nothing is the same. `?` return an error.
- using a similar syntax to functions for defining records (structs)
  - `rec: ...`
  - `rec: ... !`
- using `{{ ..., ... }}` for defining tuples
- giving default values to all primitive types (and therefore to all user-defined types)
- not being able to define methods for types (like class methods in OOP or `impl` statements in rust")

<sup><sub>(todo: examples (tho i should finish the compiler first lol))</sub></sup>

#### absolutely nothing!!!!
```
main.
```

#### hello world
```
main write "hello, world!".
```
