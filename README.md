[![Release to crates.io](https://github.com/navicore/lisplings/actions/workflows/release.yml/badge.svg)](https://github.com/navicore/lisplings/actions/workflows/release.yml)
[![lisplings](https://img.shields.io/crates/v/lisplings.svg?label=lisplings)](https://crates.io/crates/lisplings)

# Lisplings

Interactive exercises for learning [SeqLisp](https://github.com/navicore/seq-lisp), a Lisp dialect.

Inspired by [Rustlings](https://github.com/rust-lang/rustlings) and *The Little Lisper*, Lisplings teaches Lisp syntax and thinking through hands-on exercises you complete in your editor. Watch mode gives you REPL-like feedback — every time you save, you see your program's output instantly.

## Prerequisites

You need `seqlisp` installed and available in your PATH.

See the [seq-lisp installation instructions](https://github.com/navicore/seq-lisp#installation) for setup details.

## Quick Start

```bash
cargo install lisplings
lisplings init my-lisplings
cd my-lisplings
lisplings
```

This starts **watch mode** — Lisplings monitors your exercise files and shows output as you edit them.

## How It Works

1. Lisplings shows you the current exercise and its output
2. Open the exercise file in your editor
3. Read the instructions and fix/complete the code
4. Save — your program's output appears instantly in the watch frame
5. Delete the `; I AM NOT DONE` marker when finished
6. Lisplings verifies and advances to the next exercise

## Commands

| Command | Description |
|---------|-------------|
| `lisplings` | Start watch mode (default) |
| `lisplings init [dir]` | Create a new exercise project |
| `lisplings list` | Show all exercises with completion status |
| `lisplings hint` | Get a hint for the current exercise |
| `lisplings hint <name>` | Get a hint for a specific exercise |
| `lisplings verify` | Check all exercises at once |
| `lisplings next` | Skip to the next exercise |
| `lisplings reset` | Reset current exercise to original state |
| `lisplings reset <name>` | Reset a specific exercise |
| `lisplings list -c 03` | Filter exercises by chapter |
| `lisplings watch -c 05` | Watch only a specific chapter |

## Curriculum

The exercises progress from basics to higher-order programming, inspired by *The Little Lisper*:

| Chapter | Topics |
|---------|--------|
| **00-intro** | Hello world, comments, arithmetic |
| **01-atoms** | Numbers, strings, booleans, symbols |
| **02-lists** | `list`, `cons`, `car`, `cdr`, nested lists, predicates |
| **03-define** | Variables, functions, multiple arguments, composition |
| **04-conditionals** | `if`, type predicates, `cond`, nested conditionals |
| **05-recursion** | Base cases, length, sum, member, reverse with accumulator |
| **06-higher-order** | `lambda`, `map`, `filter`, `fold`, closures |
| **07-let-and-closures** | `let` bindings, nested `let`, closures, function factories |

## SeqLisp Basics

SeqLisp is a Lisp with familiar syntax — parenthesized prefix notation:

```lisp
;; Arithmetic uses prefix notation
(+ 2 3)          ;; => 5
(* (+ 1 2) 4)    ;; => 12

;; Define variables
(define x 42)

;; Define functions
(define (square x) (* x x))
(square 5)        ;; => 25

;; Lists
(list 1 2 3)               ;; => (1 2 3)
(car (list 10 20 30))      ;; => 10
(cdr (list 10 20 30))      ;; => (20 30)

;; Higher-order functions
(map (lambda (x) (* x 2)) (list 1 2 3))   ;; => (2 4 6)
(filter (lambda (x) (> x 2)) (list 1 2 3 4))  ;; => (3 4)

;; Closures
(define (make-adder n)
  (lambda (x) (+ x n)))
(define add10 (make-adder 10))
(add10 5)         ;; => 15
```

## Tips

- **Read the comments** — Each exercise explains the concept
- **Watch the output** — Your code runs on every save, even before removing the marker
- **Use `lisplings hint`** — When stuck, get a hint
- **Check solutions/** — Reference solutions are available
- **Experiment** — Try variations in the SeqLisp REPL (`seqlisp`)

## Contributing

Found a bug or want to improve an exercise? PRs welcome!

If you find gaps in the SeqLisp language itself, please open issues at [seq-lisp](https://github.com/navicore/seq-lisp).

## License

MIT
