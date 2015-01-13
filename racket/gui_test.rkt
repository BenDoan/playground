(require racket/gui/base)

(define frame (new frame% [label "Test"]))

(define msg (new message% [parent frame]
                 [label "No events so far..."]))

(new button% [parent frame]
    [label "click"]
    [callback (lambda (button event)
                     (send msg set-label "clicked"))])

(send frame show #t)

