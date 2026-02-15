Same pattern as `my-length`, but add `(car lst)` instead of 1:
```lisp
(if (null? lst)
    0
    (+ (car lst) (my-sum (cdr lst))))
```
