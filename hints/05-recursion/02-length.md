```lisp
(if (null? lst)
    0
    (+ 1 (my-length (cdr lst))))
```
If the list is empty, length is 0. Otherwise, it's 1 plus the length of the rest.
