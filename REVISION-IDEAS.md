## For next time I teach this lecture

Sketchy ideas, some sourced from Claude and Codex feedback.

- I was asking students to make a leap and point out why `or` can't just be a function, and what might go wrong with the unhygienic `tmp` version. Perhaps instead I could focus my questions on asking students to follow the mechanics and develop their understanding of intended behavior. E.g:
   * What should `(or #f 2)` and `(or 1 2)` do?
   * How do we define this as a function?
   * What should `(or 1 (error))` do?
   * How many times should the print happen in `(or (begin (displayln "here") 1) 2)`?
   * What happens with those examples when we define it as a function?
   * What happens when we define a macro that looks just the same---why is the result different?
      + Work through the code it expands to and its runtime behavior

- `or` is smaller and less painful to write by hand with `sexp-transformer` than `for/list` is, so it might make sense to start there instead. 
   * To motivate syntax-rules, maybe just follow the same pattern myself to write the `sexp-transformer` version and lightly explain; then show that it can be a lot more concise.
   * Then maybe go back to `or` afterwards and say hey, what if we try that hygiene problem one with syntax-rules? It works!? There's actually a data representation thing going on too...
      - Not so sure about this ordering though. Maybe good to return and reinforce, or maybe a problem to split up.

- An `sexp-transformer` version of `or` actually exhibiting the hygiene problem would be more interesting than just saying hypothetically what would go wrong with naive expansion but showing only `syntax-rules`. It's short if you use quasiquote:
```
(define-syntax my-or
  (sexp-transformer
   (lambda (stx)
     `(let ([tmp ,(second stx)])
        (if tmp tmp ,(third stx))))))
```

- Explore the “why can’t this be a function?” question for `for/list` as well.

- Consider putting some applications / motivations up front somehow. Maybe I can set up a want as a problem that macros / extending desugaring can solve?