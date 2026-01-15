<sup><sub><sup><sub> note: everything is in lowercase or uppercase because of the vibes i want the language to have: unserious, informal and friendly. also i won't take responsibility for the disgust, anger, frustration, or anything else you may feel reading this text :Þ </sub></sup></sub></sup>

# seabun!

<sup><sub>(todo: logo image)</sub></sup>

## introduction

seabun (lowercase everything!) is a somewhat silly programming language i made for fun :3

i designed it to be as dumb but also somewhat readable as i could make it

## design philosophy

seabun aims to be _different_ (not better, not cool, just different), breaking some common practices for the sole purpose of diverging from "the norm"

some of these "strange" design choices include:

- using a period (.) instead of a semicolon (;)
- making lambdas the default function type
- using `: ... !` or `: ... ;` instead of `( ... )` when calling or defining functions
  - `let name = fun: arg_name type, ...! body`
  - `let name = fun type: arg type, ...! body`
  - `name: param1, ...!.`
- using a similar syntax to functions (`rec: ... !`) for defining records
- using `{{ ..., ... }}` for defining tuples
- giving default values to all primitive types (and therefore to all user-defined types)
- not being able to define methods for types (like class methods in OOP or `impl` statement in rust")

(todo: examples (tho i should finish the compiler first lol))
