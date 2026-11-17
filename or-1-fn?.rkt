#lang racket


;; (or <expr> <expr>)
;; Evaluate the first expression. If it does not produce false,
;; return that value. Otherwise evaluate the second expression.
#;(define-syntax my-or
  TODO)


;; Examples:

(my-or 1 5)  ; => 1
(my-or "hello" "world")  ; => "hello"
(my-or #f 5) ; => 5




#;#;#;
(define (all-positive? l)
  (my-or (empty? l)
      (and (positive? (first l))
           (all-positive? (rest l)))))

(all-positive? '(1 5 3))  ; => #t
(all-positive? '(1 -8 3)) ; => #f








;; Can it be a function?


