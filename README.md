<sup><sub><sup><sub> note: almost everything is exclussively in lowercase or uppercase because of the vibes i want the language to have: unserious, informal and friendly. also i won't take responsibility for the disgust, anger, frustration, or anything else you may feel reading this text :Þ </sub></sup></sub></sup>

# seabun!

<sup><sub>(todo: logo image)</sub></sup>

-----

## introduction

seabun (lowercase everything) is a somewhat silly programming language i'm making for fun :3

i'm designing it to be as low-level-but-also-readable as i can, and i'm planing to make it compile to c-compatible LLVM IR

### design philosophy

seabun aims to be _different_ (not better, not cool, just different), breaking some common practices for the sole purpose of diverging from "the norm" (and because i want to heheheh)

some of these "strange" design choices include:

- having main be a keyword followed by a single expression (similar to a label)
  - `main ...`
  - `main { ... }`
- using a period (.) instead of a semicolon (;)
- using `¬` instead of `!` for "not"
- making lambdas the default function type
- using `: ...`, `: ... !` and `: ... ?` instead of `( ... )` when calling and defining functions
  - `let f = fun: x T, ... do ...`
  - `let f = fun: x T, ... { ... }`
  - `let f = fun: x T, ... -> T do ...`
  - `let f = fun: x T, ... -> T { ... }`
  - `f: x, ... !`
  - `f: x, ... ?`
  - `f: x, ...`
    - `!` can only be dropped when the call is by itself (not "nested"). that means `first: second!` is the same as `first: second!!`, as the `!` belongs to the inner call
    - for function declarations, `!` and nothing is the same. `?` tells the compiler a call may return an error<sup>(i'm still deciding which way to go with this)</sup>.
- using a similar syntax to functions for defining records (structs)
  - `rec: ...`
  - `rec: ... !`
- using `{{ ..., ... }}` for defining tuples
- using `<| ... |>` for defining enums
- giving default values to all primitive types (and therefore to all user-defined types)
- not being able to define methods for types (like class methods in OOP or `impl` statements in rust")

... and many more that can be seen in `ideas/seabun.cbun` and `ideas/random_ideas.cbun`

-----

## examples

<sup><sub>(todo: examples (tho i should finish the compiler first lol))</sub></sup>

#### absolutely nothing!!!!
```
main.
```

#### still absolutely nothing!!1!1!
```
main {}
```

#### hello world
```
main show "hello, world.\n".
```

#### hello world v2
```
main {
    let hello = "hello, ".
    let world = "world.\n".
    show hello, world.
}
```

#### "`itoa`!?!?" yes, `itoa`
```
; "as str" on numeric values ALWAYS acts as a conversion to an alphanumeric representation.
; to get a character from a code point instead, use "as chr" or "as n8" (like c's and c++'s chars).
main {
    show 123 as str. ; -> "123"
    show 123. ; -> "{"
    ; -> 123{
}
```
<sup><sub>this would be implemented as:</sub></sup>
<sup><sub>1. if it's a num/unum and is between -9 (for nums) and 9, add 48 to it's absolute and put the sign if needed</sub></sup>
<sup><sub>2. if it's a chr, wrap it in an array (strs are basically arrays of chars)</sub></sup>
<sup><sub>3. if it's a bln, write "true" or "false"</sub></sup>
<sup><sub>4. for other types, either call (typename)_stringify, use the underlying type's method (aliases), or panic</sub></sup>
<sup><sub>5. arrays and tuples just apply it to all "as str" to all elements</sub></sup>
<sup><sub>either that or just syntactic sugar for point 4 for all types</sub></sup>
<sup><sub>... but this is just a rough idea i guess</sub></sup>

