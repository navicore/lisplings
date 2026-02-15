Nest two `let` expressions:
```lisp
(let x 3 (let y 4 (+ (* x x) (* y y))))
```
The inner body can see both `x` and `y`.
