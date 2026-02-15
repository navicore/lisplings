You cannot pass bare `+` to fold. Wrap it in a lambda:
```lisp
(fold (lambda (a b) (+ a b)) 0 (list 1 2 3 4 5))
```
