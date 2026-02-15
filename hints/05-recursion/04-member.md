Three cases:
1. Empty list: return `#f`
2. `(car lst)` equals `x`: return `#t`
3. Otherwise: recurse on `(cdr lst)`

Use `cond` or nested `if`.
