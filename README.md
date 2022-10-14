# Mathr

A small math interpreter implemented with Rust.
I followed this [link](https://ruslanspivak.com/lsbasi-part1/) and implemented the whole interpreter.
It's a very useful tutorial if someone is interested in how an interpreter works.

## Spec

- Number: usigned 32-bit integer
- Binary Operations: `+, -, *, /`
- Built-in Functions:
  - log, log2, log10, ln
  - pow, pow2, pow10
  - sqrt
  - ceil, floor, round
- Built-in Symbol:
  - e
  - pi

## Grammar

```
statement_list : statement
               | statement SEMI statement_list

statement : assignment_statement

assignment_statement : variable ASSIGN expr

empty :

expr: term ((ADD | SUB) term)*

term: factor ((MUL | DIV) factor)*

factor : ADD factor
       | SUB factor
       | NUMBER
       | LPAREN expr RPAREN
       | variable

variable: ID
```
