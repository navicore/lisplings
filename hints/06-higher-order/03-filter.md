Use `filter` with a lambda that checks for even numbers:
```lisp
(filter (lambda (x) (equal? (modulo x 2) 0)) (list 1 2 3 4 5 6))
```
