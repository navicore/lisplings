Use nested `if` expressions:
```lisp
(if (number? x) "number"
    (if (string? x) "string"
        (if (null? x) "empty"
            "other")))
```
