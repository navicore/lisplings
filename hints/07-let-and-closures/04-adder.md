Two fixes needed:
1. Lambda body: `(+ x n)` instead of `0`
2. `result2`: use `(make-adder 5)` not `(make-adder 0)`

```lisp
(define (make-adder n)
  (lambda (x) (+ x n)))

(define result2 (let add5 (make-adder 5) (add5 10)))
```
