Return a lambda that captures `n`:
```lisp
(define (make-adder n)
  (lambda (x) (+ x n)))
```
Then set `add10` to `(make-adder 10)`.
