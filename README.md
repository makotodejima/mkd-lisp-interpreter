### A simple interpreter for a Lisp-like language

Designed to execute programs with Lisp-style syntax, focusing on simplicity and extensibility.<br/>Written in Rust. It contains basic components essential for language interpretation - tokenization, parsing into an Abstract Syntax Tree (AST), and evaluation.

Key features:
- Lisp syntax interpretation, including lists, functions, and arithmetic.
- Dynamic scoping with nested environment support.
- Basic error handling for syntax and runtime errors.
- Implemented in Rust for robustness and efficiency.

### Examples

```lisp
; Arithmetic
(+ 1 2 3)

; Variable definition
(def num 10)
(def another_num 4)
(- num another_num) ; 6

; Functions
(def add (fn (a b) (+ a b)))
(add 3 4) ; 7

; Conditional
(if (> 1 0) true false) ; true
(if (> num another_num) (add num another_num) 0) ; 14

```

---

The project is inspired by the following articles:
- http://norvig.com/lispy.html
- https://stopa.io/post/222
