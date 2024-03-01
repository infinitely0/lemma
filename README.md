# Lemma

Toy programming language written to learn about compilers and Rust. Basics are implemented:
lexical analysis, parsing, AST evaluation, scoping, types, function calls, and error handling.

## Examples

Run with `cargo run -- examples/fizzbuzz.lm`

```
# Prints the first 100 terms of the fizz buzz sequence

-> Void
main = Int [ 1 .. 100 ] >> fizzbuzz

Int -> Str
fizzbuzz n = if == (rem n 15) 0
               then "fizzbuzz"
             else if == (rem n 3) 0
               then "fizz"
             else if == (rem n 5) 0
               then "buzz"
             else n

Int Int -> Int
rem a b = - a (* b (/ a b))
```

The syntax uses ubiquitous prefix notation. The symbol `>>` is not a bit shift but a pipe operator,
like `%>%` in R, and similar to `$` in Haskell but in the opposite direction.

## EBNF

Context-free grammar in extended Backus–Naur form:

```
Program = { Function }

Function = Signature , Definition

Signature = { Type } "->"  Type
Definition = Identifier [ { Identifier } ] "=" Expression

Expression = Call
           | Value
           | Conditional

Call = Identifier { Argument }
     | Operator { Argument }

Value = Integer
      | Fractional
      | String
      | Boolean

Conditional = if Expression then Expression else Expression

Identifier = Letter { Letter | Digit | "-" }
Operator = "+" | "-" | "*" | "/"
         | "==" | "!="
         | "<" | ">" | "<=" | ">="
         | "&&" | "||"
         | "!"

Argument = Value
         | Identifier
         | "(" Expression ")"
 
Integer = { Digit }
Fractional = { Digit } "." { Digit }
String = "'" Character "'"
Boolean = "true" | "false"
Character = [a-zA-Z]
Digit = [0-9]
```

