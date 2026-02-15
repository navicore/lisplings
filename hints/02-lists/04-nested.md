The nested list is `((a b) (c d))`.
- `(car nested)` gives `(a b)`
- `(cdr (car nested))` gives `(b)`
- `(car (cdr (car nested)))` gives `b`
