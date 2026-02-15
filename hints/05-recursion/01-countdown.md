```lisp
(define (countdown n)
  (if (<= n 0)
      '()
      (cons n (countdown (- n 1)))))
```
Base case: return empty list when n <= 0.
Recursive case: cons n onto the countdown of (n - 1).
