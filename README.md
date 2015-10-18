# Rust Guessing Game

>Our program will generate a random integer between one and a hundred. It will then prompt us to enter a guess. Upon entering our guess, it will tell us if we’re too low or too high. Once we guess correctly, it will congratulate us.

## Glossary

### [Tuples](https://doc.rust-lang.org/book/primitive-types.html#tuples)

A tuple is an ordered list of fixed size. Like this:

```
let x = (1, "hello");
```

Here’s the same code, but with the type annotated:

```
let x: (i32, &str) = (1, "hello");
```

#### Arity

>Arity refers to the number of arguments a function or operation takes.

- A nullary function takes no arguments.
- A unary function takes one argument.
- A binary function takes two arguments.
- A ternary function takes three arguments.
- An n-ary function takes n arguments.
