The helper takes `remaining` and `acc` (accumulator):
- If remaining is empty, return acc
- Otherwise, recurse: move `(car remaining)` to front of acc

```lisp
(define (reverse-helper remaining acc)
  (if (null? remaining)
      acc
      (reverse-helper (cdr remaining) (cons (car remaining) acc))))
```
