###### everything is in lowercase because of the vibes i want the language to have: unserious, informal and friendly
###### also i won't take responsibility for the disgust, anger, frustration, or anything else you may feel reading this text :Þ

(todo: logo image)

# _introduction_

seabun (lowercase everything!) is a somewhat silly programming language i made for fun :3

i designed it to be as dumb while also somehwt readable as i could make it

# _design philosophy_
seabun aims to be _different_ (not better, not cool, just different), breaking some common practices for the sole purpose of diverging from "the norm"

some of these "strange" design choices include:
- using a period (.) instead of a semicolon (;)
- using `: ... !` or `: ... ;` instead of `( ... )` when calling or defining functions and lambdas
- using `{{ ... }}` for defining records
- using `[ ... ]` for defining tuples
- making lambdas the default function type (makes for a more natural reading imo)
    let a = fun: x num! { say (x + 64) as str. }.
    let b = fun num: x num! { give x + 65 }.
- giving default values to all primitive types (and therefore to all user-defined types)
- not being able to define  functions (like class functions in OOP or `impl` statement in rust")

(todo: examples (tho i should finish the compiler first lol))
