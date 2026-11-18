#lang racket

;; Macros written with list functions like `first`, `rest`, etc
;; are pretty hard to read...

;; What if we had a specialized language for writing macros?
;; How *should* they look?


(define-syntax for/list
  (syntax-rules ()
    [(for/list ([var rhs] ...)        ; Pattern
       body)
     (map (lambda (var ...) body)     ; Template
          rhs ...)]))


(for/list ([x '(1 2 3)])
  (+ x 1))





;; What if we want to iterate over multiple lists, together?
(for/list ([expected '(1 2 3)]
           [actual   '(1 3 3)])
  (equal? expected actual))



;; Equivalent to:

(map (lambda (expected actual)
       (equal? expected actual))
     '(1 2 3)
     '(1 3 3))