The lambda body should use `factor`:
```lisp
(define (make-multiplier factor)
  (lambda (x) (* factor x)))
```
The returned lambda "closes over" factor, remembering its value.
