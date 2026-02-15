Use nested `if`:
```lisp
(if (< n lo) lo
    (if (> n hi) hi
        n))
```
